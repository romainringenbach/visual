use crate::engine::Engine;
use crate::project::Project;

use std::{time::Instant};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        PrimaryCommandBufferAbstract, RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    format::Format,
    image::{
        view::ImageView, ImageAccess, ImageDimensions, ImmutableImage, MipmapsCount,
        SwapchainImage,
    },
    memory::allocator::{AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            vertex_input::Vertex,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
    swapchain::{
        acquire_next_image, AcquireError, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow},
    window::{Window},
};
use crate::midi::listen;
use vs::PushConstants;

use std::sync::{Arc, Mutex};

pub fn run(project : &Project) {
    let mut engine = Engine::new();

    let memory_allocator = StandardMemoryAllocator::new_default(engine.device.clone());

    // We now create a buffer that will store the shape of our triangle. We use `#[repr(C)]` here
    // to force rustc to use a defined layout for our data, as the default representation has *no
    // guarantees*.
    #[derive(BufferContents, Vertex)]
    #[repr(C)]
    struct Vertex {
        #[format(R32G32_SFLOAT)]
        position: [f32; 2],
    }

    let vertices = [
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
    ];
    let vertex_buffer = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vertices,
    )
        .unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        engine.device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: engine.swapchain.image_format(),
                samples: 1,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
        .unwrap();

    let vs = vs::load(engine.device.clone()).unwrap();
    let fs = (project.frag_loader)(engine.device.clone()).unwrap();

    let push_constants = Arc::new(Mutex::new(PushConstants{
        note : [0,0,0,0,0,0,0,0],
        velocity : [0,0,0,0,0,0,0,0],
        time: 0,
    }));

    let current_time = Instant::now();

    let w_midi_push_constants = push_constants.clone();

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let mut _conn_in = listen(move | channel, note, velocity, |{
        println!("DEBUG : channel[{0}] : ({1},{2})",channel,note,velocity);

        if channel >= 8 {
            return;
        }

        let mut p = w_midi_push_constants.lock().unwrap();
        p.note[channel] = note;
        p.velocity[channel] = velocity;

    });

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(engine.device.clone());
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(engine.device.clone(), Default::default());
    let mut uploads = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        engine.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
        .unwrap();

    let texture = {
        let png_bytes = include_bytes!("Noise.png").as_slice();
        let decoder = png::Decoder::new(png_bytes);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize(info.width as usize * info.height as usize * info.bytes_per_pixel(), 0);
        reader.next_frame(&mut image_data).unwrap();

        let image = ImmutableImage::from_iter(
            &memory_allocator,
            image_data,
            dimensions,
            MipmapsCount::One,
            Format::R8_SRGB,
            &mut uploads,
        )
            .unwrap();
        ImageView::new_default(image).unwrap()
    };

    let sampler = Sampler::new(
        engine.device.clone(),
        SamplerCreateInfo {
            mag_filter: Filter::Linear,
            min_filter: Filter::Linear,
            address_mode: [SamplerAddressMode::Repeat; 3],
            ..Default::default()
        },
    )
        .unwrap();

    let pipeline = GraphicsPipeline::start()
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .vertex_input_state(Vertex::per_vertex())
        .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip))
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .build(engine.device.clone())
        .unwrap();

    let layout = pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [WriteDescriptorSet::image_view_sampler(0, texture, sampler)],
    )
        .unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let mut framebuffers = window_size_dependent_setup(&engine.images, render_pass.clone(), &mut viewport);
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(engine.device.clone(), Default::default());

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(
        uploads
            .build()
            .unwrap()
            .execute(engine.queue.clone())
            .unwrap()
            .boxed(),
    );

    let r_vulkan_push_constants = push_constants.clone();

    engine.event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {
                let window = engine.surface.object().unwrap().downcast_ref::<Window>().unwrap();
                let dimensions = window.inner_size();
                if dimensions.width == 0 || dimensions.height == 0 {
                    return;
                }

                previous_frame_end.as_mut().unwrap().cleanup_finished();

                if recreate_swapchain {

                    let (new_swapchain, new_images) =
                        match engine.swapchain.recreate(SwapchainCreateInfo {
                            image_extent: dimensions.into(),
                            ..engine.swapchain.create_info()
                        }) {
                            Ok(r) => r,
                            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                            Err(e) => panic!("failed to recreate swapchain: {e}"),
                        };

                    engine.swapchain = new_swapchain;

                    framebuffers = window_size_dependent_setup(
                        &new_images,
                        render_pass.clone(),
                        &mut viewport,
                    );

                    recreate_swapchain = false;
                }

                let (image_index, suboptimal, acquire_future) =
                    match acquire_next_image(engine.swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire next image: {e}"),
                    };

                if suboptimal {
                    recreate_swapchain = true;
                }

                let mut builder = AutoCommandBufferBuilder::primary(
                    &command_buffer_allocator,
                    engine.queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                    .unwrap();

                let mut p = r_vulkan_push_constants.lock().unwrap().clone();
                p.time = current_time.elapsed().as_millis() as u32;

                builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],

                            ..RenderPassBeginInfo::framebuffer(
                                framebuffers[image_index as usize].clone(),
                            )
                        },
                        SubpassContents::Inline,
                    )
                    .unwrap()
                    .set_viewport(0, [viewport.clone()])
                    .bind_pipeline_graphics(pipeline.clone())
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        set.clone(),
                    )
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .push_constants(pipeline.layout().clone(), 0, p)
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass()
                    .unwrap();

                let command_buffer = builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(engine.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        engine.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(engine.swapchain.clone(), image_index),
                    )
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("failed to flush future: {e}");
                        previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                    }
                }
            }
            _ => (),
        }
    });
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
                .unwrap()
        })
        .collect::<Vec<_>>()
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/vert.glsl",
    }
}