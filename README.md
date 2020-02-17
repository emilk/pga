# Projective Geometric Algebra

This library attempts to define useful functionality of 2d Project Geometric Algebra (PGA) in a type safe way, meaning all basis vector (e1, e2 etc) cannot accidentally be mixed up. It also will define useful types like Point and Line, aiming for specificity rather than generality.

It is also made to teach myself the fundamentals of PGA.

# Inspiration
[The Geometric Algebra course at Siggraph 2019](https://www.youtube.com/watch?v=tX4H_ctggYo)
https://bivector.net/index.html

http://terathon.com/blog/projective-geometric-algebra-done-right/
http://terathon.com/pga_lengyel.pdf

# Goal

### Algebra
You first specify the the algebra by the signs of the product of the base vectors, e.g. 0++ for 2d projective geometric algebra, which would be the base vectors `e0^2=0, e1^2=1, e2^2=1`.

From these all products are generated, creating bivectors (`e01`, `e12` etc), trivectors (`e012` etc), and so on. Together with the Real type they make of the `blades` of the system. All values are a linear combination of the blades, e.g. `0.5 + 2*e3 - 42*e013`.

### Named blade groups (types)
We can give names to groups of blades, for instance naming `line={e0,e1,e2}`  `point={e01,e20,e12}`. In this case, any value which only have

In 2d projective algebra the types are:

`multi: r e0 e1 e2 e01 e20 e12 e123`
`point: e01 e20 e12`
`line:  e0 e1 e2`

Note that a value can have multiple types. The types thus form a Venn diagram.

These types will be combines against each other for all operations, unary (like the dual) as well as binary (multiplication, dot product, wedge, regressive, ...). The generator will notice what dimensions (blades) will be the output, and deduce a type name form that. For instance, `line ^ line = point` (TODO: check)
