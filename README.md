# Projective Geometric Algebra

This library is a Rust code generator, generating the mathematics you need for a geometric algebra library.

I made it mostly to teach myself the fundamentals of PGA.

Consider it an experiment.

# Inspiration
[The Geometric Algebra course at Siggraph 2019](https://www.youtube.com/watch?v=tX4H_ctggYo)
https://bivector.net/index.html

Foundation of Game Engine Development, Volume 1, by Eric Lengyel
http://terathon.com/blog/projective-geometric-algebra-done-right/
http://terathon.com/pga_lengyel.pdf


## 3D example - projective

**Vector names**: `X Y Z W`

**Grammar**: `X²=1  Y²=1  Z²=1  W²=0`

**Types**:

| Name         | Blades                          | Alternative | Interpretation                           |
| ------------ | ----------------------------- | ----------- | ---------------------------------------- |
| Vec3         | `X Y Z`                       |             |  direction
| Vec4         | `X Y Z W`                     |             |  homogenous point
| Point3       | `X Y Z W=1`                   |             |  normalized point
| Line3        | `WX WY WZ  YZ ZX XY`          |             |  direction + moment of a Plücker line
| Plane        | `YZW ZXW XYW  ZYX`            | !(X Y Z  W) |  normal + offset
| Translator3  | `YZ ZX XY XYZW`               |             |  translation primitive
| Rotor3       | `WX WY WZ XYZW`               |             |  rotation primitive, a.k.a. quaternion
| Motor3       | `WX WY WZ XYZW YZW ZXW XYW S` |             |  translator + rotor, a.k.a. dual quaternion

From the above definition, this library generates all the operations that can be done on these types. For instance, it will autmatically realize that `Point3 ^ Point3 -> Line3` (wedging two points gives the line that goes through those points) and `Plane V Line3 -> Vec4` (the antiwedge of a plane and a line is the point where the plane and line interesect).

The generated code uses newtypes for all vectors and blades, so that `x = y;` wouldn't compile (since `x` and `y` coordinates run along different vectors).

# Details

## Algebra
You first specify the the algebra by the signs of the product of the base vectors, e.g. `0++` for 2d projective geometric algebra, which would be the base vectors `e0²=0, e1²=1, e2²=1`.

You can also give your base vectors more descriptive names, e.g. `X/Y/Z/W` for your standard homogeneous 3D PGA.

From these all products are generated, creating bivectors (`XY`, `YZ` etc), trivectors (`XYZ` etc), and so on. Together with the `R`eal type they make up the *blades* of the system. All values are a linear combination of the blades, e.g. `0.5 + 2*X - 42*XZ`.

### Named blade groups (types)
We can give names to groups of blades, for instance naming `Vec3={X,Y,Z}`, `Point3={X,Y,Z,W}`, `Line3={WX, WY, WZ, YZ, ZX, XY}` ([Plücker coordinates](https://en.wikipedia.org/wiki/Pl%C3%BCcker_coordinates)), `Plane={YZW,ZXW,XYW,ZYX}` etc.

Note that a value can have multiple types. For instance, in the example above, any `Vec3` or also a `Point3` with `W=0` (an infinite point). The types thus form a Venn diagram.

These types will be combined against each other for all operations, unary (like the dual) as well as binary (multiplication, dot product, wedge, regressive, ...). The generator will notice what dimensions (blades) will be the output, and deduce a type name form that. For instance, `Point3 ^ Point3 -> Line3` (wedging two points gives you the line that goes through both points) or `Plane V Line3 -> Point3` (the antiwedge of a plane and a line is the point where the plane and line interesect).


# A very brief introduction to Geometric Algebra
As a programmer, my view of Geometric Algebra is as a type safe superset of linear algebra that unifies many differents parts of the standard 3D programming toolset into one theory. Using GA we can combine vectors, points, plücker lines, planes, translators, rotors (quaternions) and motors (dual quaternions) into one framework. This library generates the code for these primitves and all valid operations you can do using them.

## Notation

Geometric Algebra (GA) is great and very general. We will focus first on the interesting subset of 3D Projective Geometric Algebra (PGA). This is where you denote point and vectors with an extra projective dimension `w` and define `w=0` as directions and `w=1` as cartesian coordinates. Thus a vector is expressed as `[x, y, z, w]`.

In textbook geometric algebra, the base vectors are given the names `e1/e2/e3/e4` (or sometimes `e0/e1/e2/e3`). Due to this unfamiliarity and inconsistency I prefer to simply rename them to the familiar names `X/Y/Z/W`.

So, I use this notations:

2D PGA:

| Class     | Blades    | Description | |
| --------- | --------- | ----------- | -- |
| Scalar    | S         | Numbers     | The set of real numbers
| Vectors   | X  Y  W   | Directions  | Orthogonal dimensions in projective space
| Bivectors | YW WX XY  | Lines       | Orthogonal planes with normals `X Y W`
| Trivector | XYW       | Area        |

I will use lower case letters for `x,y,z,w` for values (e.g. `3.14`) and upper case letters `X,Y,Z,W` for the basis vectors, which can be thought of as the *type* of the value.

## Vectors and planes
Consider two arrows on the ground, both with heir names engraved on them: `e` and `n`, and they happen to be pointing east and north. What can you do with two arrows?

```
   e          ^
------>       |
             n|
              |
```

One thing you can do with them, is to stick them together at their bases. We call this *wedging*:

```
 ^
 |
n|
 +------>
     e
```

In geometric algebra, we write this as `n^e`. If one arrow has length 2 and the other length 3 we get `2N^3E` = 6N^E = 6NE`. This represents the plane spanned by the two vectors, with the magnitude representing the area of the parallelogram spanned by the vectors.


## Thinking in geometric algebra.
The value `x` means "How much along the `X` axis?".

`XY`: How much area do I have in the plane spanned by `XY` plane, i.e. the plane pointing along the positive `Z` axis.

Say you have three orthogonal vectors which are the sides of a cube. What is the volume of them together?

Classical thinking would get you `|v0|*|v1|*|v2|`. In GA, you instead think of the dimensions involved.

Each vector has three scalars: `{x: X, y: Y, z: Z}` (`w=0` since these are directions).
We want to get to the volume, which has dimension `XYZ`. We want to from low dimension (directions) to a higher one (volume), so what do we do? We wedge them together! Wedging is joining many lower-dimensions things (lines) into one higher dimension things (volume). So the answer is `v0^v1^v2`. And then you are done!
