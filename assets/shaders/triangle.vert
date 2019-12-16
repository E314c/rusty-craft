#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

out VERT_SHAD_OUTPUT {
    vec2 TexCoord;
} OUT;

uniform float timed_colour; // FOR DEBUGGING ROTATION

uniform vec3 scale;
uniform vec3 translation;
uniform vec3 rotation;

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

// Apply the given perspective to the given coordinates
vec4 applyPerspective(vec3 coord, float fov, float near, float far, float aspectRatio) {
    if(far == near) return vec4(-2,-2,-2, 2);   // Error, draw out of bounds
    
    float arcTanFov = atan(fov/2);


    vec4 newCoords;

    newCoords[3] = (arcTanFov * near * far)/(coord[2] - near);

    newCoords[0] = coord[0];
    newCoords[1] = coord[1] * aspectRatio;
    newCoords[2] = ((far + near - (2 * coord[2]))/(far - near)) * ((arcTanFov * near * far)/(coord[2] - near));

    return newCoords;
}

// Rotates in the order X->Y->Z
vec3 applyRotation(vec3 coord, vec3 rotations) {
    return rotate(
        rotate(
            rotate(
                coord,
                vec3(1,0,0),
                rotations[0]
            ),
            vec3(0,1,0),
            rotations[1]
        ),
        vec3(0,0,1),
        rotations[2]
    );
}

// -- MAIN -- //
void main()
{
    
    // // From global transforms:
    vec3 pos = applyRotation(Position, rotation); 
    gl_Position = applyPerspective(pos, asin(1),2.0, -2.0, 8.0/4.5); // TODO: Move to uniform values.

    OUT.TexCoord = TexCoord;
}
