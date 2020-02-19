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


# TODO

``` rust
enum Op {
	/// Scalar
	S(int32),
	// NOTE: not a blade(!)
	Vec(VecIdx),
	Var(Variable),

	Neg(Box<Op>),
	Dual(Box<Op>),
	Rev(Box<Op>),

	Sum(Vec<Op>),

	Prod(Vec<Op>),
	Dot(Vec<Op>),
	Wedge(Vec<Op>),

	AntiProd(Vec<Op>),
	AntiDot(Vec<Op>),
	AntiWedge(Vec<Op>),
}

enum Type {
	/// Tuple-type, linear combination of blades
	/// Blades(vec![])       = () = Zero
	/// Blades(vec![S])      = (S)
	/// Blades(vec![S, E02]) = (S, E02)
	Blades(Vec<Blade>),
	Struct(Struct),
}

/// named members
enum Struct(Vec<String, Type>)

/// In order of preference (first match).
struct Types(Vec<NamedType>)

impl Op {
	fn zero() -> Op { Op::Scalar(0) }
	fn one() -> Op { Op::Scalar(1) }

	fn typ(&self) -> Type;
	fn expand(self) -> Op { ... }
	fn simplify(self, g: &Grammar) -> Op { ... }
}



// All is data. All operations is putting together the data and calling `eval()` which simplifies as far as it can.

fn test() {
	/// pga2d
	let g = Grammar::pga2d();
	let p = g.typ("point").instance("l");
	let l = g.typ("point").instance("a");

	let op: Op = Op::Prod(vec![Op::Var(p), Op::Var(l)]);

	assert_eq_ignore_spaces!(op.rust(), "l ^ a");

	assert_eq_ignore_spaces!(op.expand().rust(),
	    r#"
	    	l.x ^ a.x + l.x ^ a.y + l.x ^ a.w +
	    	l.y ^ a.x + l.y ^ a.y + l.y ^ a.w +
	    	l.w ^ a.x + l.w ^ a.y + l.w ^ a.w
	    "#);

	assert_eq_ignore_spaces!(op.expand().simplify(g).rust(),
	    r#"
	    	            l.x ^ a.y + l.x ^ a.w +
	    	l.y ^ a.x +             l.y ^ a.w +
	    	l.w ^ a.x + l.w ^ a.y
	    "#,
	    ");

	assert_eq_ignore_spaces!(op.expand().simplify(g).typed(t).rust(),
	    r#"
	        Line {
		    	yw: l.y ^ a.w + l.y ^ a.w,
		    	wx: l.x ^ a.w + l.w ^ a.x,
		    	xy: l.x ^ a.y + l.y ^ a.x,
		    }
	    "#);
}
```





# Tutorial On Geometric Algebra
## Notation

Geometric Algebra (GA) is great and very general. We fill focus first on the interesting subset of 3D Projective Geometric Algebra (PHA). This is where you denote point and vectors with an extra projective dimension `w` and define `w=0` as directions and `w=1` as cartesian coordinates. Thus a vector is expressed as `[x, y, z, w]`.

GA replaces the familiar named with `x/y/z/w` with the less familiar `e1/e2/e3/e4` or sometimes `e0/e1/e2/e3`. Due to this unfimiliarity and unconsistency I prefer to simply rename them to the familiar names `x/y/z/w`.

So, I use this notations:
```
2D PGA:


| Class     | Blades    | Description |
| --------- | --------- | ----------- |
| Scalar    | S         | The set of real numbers
| Vectors   | X  Y  W   | Orthogonal dimensions in projective space
| Bivectors | YW WX XY  | Orthogonal planes with normals `X Y W`
| Trivector | XYW       | Represents a volume

3D PGA

Scalar:     S           Really the number 1
Vectors:    X Y Z W     Orthogonal dimensions
Bivectors:  XY XZ XW
```

I will use lower case letters for `x,y,z,w` for values (e.g. `3.14`) and upper case letters `X,Y,Z,W` for the basis vectors, which can be thought of as the *type* of the value.

## Thinking with geometric algebra.

What the values mean:

x:  How much along the X axis?

XY: How much area do I have in the plane spanned by XY plane, i.e. the plane pointing along the positive Z axis.


Say you have three orthogonal vectors which are the sides of a cube. What is the volume of them together?

Classical thinking would get you `|v0|*|v1|*|v2|`.

In GA, you instead think of the dimensions involved.

Each vector has three scalars: `{x: X, y: Y, z: Z}` (`w=0` since these are directions).
We want to get to the volume, which has dimension `XYZ`. So what do we do? We want to from many low dimension to a higher one, so what do we do? We wedge them together! Wedging is joining many lower-dimensions things (lines) into one higher dimension things (volume). So the answer is v0^v1^v2. And then you are done!

## Fingers an planes
Consider two arrows on the ground, both with heir names engraved on them: `a` and `b`, and they happen to be pointing due east and due north. What can you do to two arrows?

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
 |------>
     e

In geometric algebra, we write this as n^e. If one arrow has length 2 and the other length 3 we get 2n^3e = 6n^e.

Now consider two other arrows




```


We call this wedge a bivector. What do they represent? Consider two other sticks point
