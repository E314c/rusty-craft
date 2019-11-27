#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

out VERT_SHAD_OUTPUT {
    vec2 TexCoord;
} OUT;

uniform float timed_colour; // FOR DEBUGGING ROTATION

vec4 multiplyQuat(vec4 a, vec4 b) {

    /**
        (a + ib + jc + kd) * (e + if + jg + kh). 
    
        rules:
        i*i = j*j = k*k = -1
        i*j = k,
        j*i = -k
        j*k = i,
        k*j = -i
        k*i = j,
        i*k = -j
     */
    //  ae    + af  i + ag  j + ah  k
    // +be  i + bf ii + bg ij + bh ik 
    // +ce  j + cf ji + cg jj + ch jk 
    // +de  k + df ki + dg kj + dh kk 
    // ->
    //  ae    + af  i + ag  j + ah  k
    // +be  i + bf -1 + bg  k + bh -j 
    // +ce  j + cf -k + cg -1 + ch  i 
    // +de  k + df  j + dg -i + dh -1 
    // ->
    //     ae - bf - cg - dh 
    // + i(af + be + ch - dg)
    // + j(ag - bh + ce + df)
    // + k(ah + bg - cf + de)

    return vec4(
        (a[0] * b[0]) - (a[1] * b[1]) - (a[2] * b[2]) - (a[3] * b[3]),  // real
        (a[0] * b[1]) + (a[1] * b[0]) + (a[2] * b[3]) - (a[3] * b[2]),  // i
        (a[0] * b[2]) - (a[1] * b[3]) + (a[2] * b[0]) + (a[3] * b[1]),  // j
        (a[0] * b[3]) + (a[1] * b[2]) - (a[2] * b[1]) + (a[3] * b[0])   // k
    );
}

// Function to rotate a coordinate around a unit axis
// Note: rotation is warped if axis is not a unit length vector (eg/ if sqrt(x^2 + y^2 + z^2) != 1)
vec3 rotate(vec3 coord, vec3 axis, float angle) {
    float sinAngle = sin(angle/2);
    float cosAngle = cos(angle/2);

    vec4 q = vec4( 
        cosAngle, 
        sinAngle*axis[0],
        sinAngle*axis[1],
        sinAngle*axis[2]
    );

    vec4 qPrime = vec4(
        cosAngle,
        -sinAngle * axis[0],
        -sinAngle * axis[1],
        -sinAngle * axis[2]
    );

    vec4 p = vec4(
        0,
        coord[0],
        coord[1],
        coord[2]
    );

    vec4 res =  multiplyQuat(multiplyQuat(q,p),qPrime);

    // return the non-real parts
    return vec3(
        res[1],
        res[2],
        res[3]
    );
}

// -- MAIN -- //

void main()
{
    // Fixed position
    // gl_Position = vec4(Position, 1.0);
    
    // DEBUG: Rotating the shape around the x-axis
    vec3 pos = rotate(Position, vec3(1,0,0), timed_colour);
    gl_Position = vec4(pos, 1.0);
    
    OUT.TexCoord = TexCoord;
}