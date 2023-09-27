#version 450

#include <common.glsl>

const float basesizeX = 0.1;
float speed = 0.001;

const vec2 right = vec2(1,0);

bool isInCircleArc(float startAngle, float thin, float arcLength, float distanceToCenter, vec2 currentPosition, vec2 screenSize){
    startAngle = mod(startAngle,radians(360));
    if(startAngle < 0){
        startAngle = radians(360) -startAngle;
    }
    vec2 realPos = currentPosition;
    realPos.y *= screenSize.y/ screenSize.x;



    float c = radians(360) * distanceToCenter;
    float ac = c * arcLength / radians(360);
    float tc = (thin/2.0)/ac * arcLength;

    startAngle += tc;
    startAngle = mod(startAngle,radians(360));
    arcLength -= tc*2.0;


    if(length(realPos) >= distanceToCenter - thin/2.0 && length(realPos) <= distanceToCenter + thin/2.0){
        float angle = acos(dot(normalize(realPos),normalize(right)));
        if(realPos.y >= 0){
            angle = radians(360) - angle;
        }

        if(startAngle + arcLength <= radians(360.0)){
            if(angle >= startAngle && angle <= startAngle + arcLength){
                return true;
            }
        } else {
            if(angle >= startAngle || angle <= mod(startAngle + arcLength,radians(360))){
                return true;
            }
        }


    }

    mat2 startMat = mat2(cos(-startAngle),sin(-startAngle),-sin(-startAngle),cos(-startAngle));
    vec2 startPos = (startMat * right) * distanceToCenter;

    float endAngle = startAngle + arcLength;
    mat2 endMat = mat2(cos(-endAngle),sin(-endAngle),-sin(-endAngle),cos(-endAngle));
    vec2 endPos = (endMat * right) * distanceToCenter;

    if(length(realPos-startPos) < thin/2.0){
        return true;
    }

    if(length(realPos-endPos) < thin/2.0){
        return true;
    }


    return false;
}

const float circleSeparator = 0.01;
const float circlesThin[10] = {0.025,0.05,0.125,0.15,0.25,0.05,0.1,0.025,0.125,0.075};
const float circleDistancesToCenter[10] =    {circlesThin[0]/2.0,circlesThin[0]+circlesThin[1]/2.0+circleSeparator,circlesThin[0]+circlesThin[1]+circlesThin[2]/2.0+circleSeparator*2.0,
                                        circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]/2.0+circleSeparator*3.0,
                                        circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]+circlesThin[4]/2.0+circleSeparator*4.0,
circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]+circlesThin[4]+circlesThin[5]/2.0+circleSeparator*5.0,
circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]+circlesThin[4]+circlesThin[5]+circlesThin[6]/2.0+circleSeparator*6.0,
circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]+circlesThin[4]+circlesThin[5]+circlesThin[6]+circlesThin[7]/2.0+circleSeparator*7.0,
circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]+circlesThin[4]+circlesThin[5]+circlesThin[6]+circlesThin[7]+circlesThin[8]/2.0+circleSeparator*8.0,
circlesThin[0]+circlesThin[1]+circlesThin[2]+circlesThin[3]+circlesThin[4]+circlesThin[5]+circlesThin[6]+circlesThin[7]+circlesThin[8]+circlesThin[9]/2.0+circleSeparator*9.0};
const float circleStart[10] = {0.0,radians(90),radians(135),radians(-90),0.0,radians(180),radians(45),radians(90),0.0,radians(180)};
const float circleLength[10] = {radians(270),radians(90),radians(135),radians(90),radians(90),radians(180),radians(45),radians(90),radians(135),radians(180)};
const int circleAngle[10] = {2,4,3,2,5,4,2,5,3,4};

void main() {

    float t = getTime()*speed;

    vec4 dummy = texture(tex,iTexCoord);

    vec3 c;
    float l,z=t;
    vec2 r = vec2(getData(0),getData(1));

    float rAngle = getData(2);
    mat2 rotationMat = mat2(cos(rAngle),sin(rAngle),-sin(rAngle),cos(rAngle));

    vec2 coord = iTexCoord;
    float sin_factor = sin(rAngle);
    float cos_factor = cos(rAngle);
    coord = vec2((coord.x - 0.5) * (r.x / r.y), coord.y - 0.5) * mat2(cos_factor, sin_factor, -sin_factor, cos_factor);
    coord += 0.5;



    bool isOnCircle1 = isInCircleArc(getData(2),0.1,radians(180.0)/2.0,0.3,iPosition,r);

    bool findCircle = false;
    int circleIndex = -1;
    float startAngle = 0.0;
    for(int i = 0; i < 10 && !findCircle ; i++){
        startAngle = getData(circleAngle[i])+circleStart[i];
        bool isLeft = getData(circleAngle[circleIndex])+circleStart[circleIndex] > radians(90) && getData(circleAngle[circleIndex])+circleStart[circleIndex] < radians(270);
        if(getNote(circleAngle[circleIndex]) > 0 && ((isLeft && getNote(4) > 0) || (!isLeft && getNote(5) > 0)) ){
            startAngle += radians(180);
        }

        if(isInCircleArc(startAngle,circlesThin[i],circleLength[i],circleDistancesToCenter[i],iPosition,r)){
            findCircle = true;
            circleIndex = i;
        }
    }

    vec3 d = vec3(0.0);
    if(findCircle){
        d = vec3(1.0);

    }

    if(getNote(6) > 0 && rand(iPosition) > 0.8){
        d += vec3(dummy.r) * 0.5;
    }


    f_color = vec4(d,1.0);
}