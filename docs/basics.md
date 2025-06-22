# Basic Program

```
#[start]
fn main() {
    print("Hello, world!")
    x = 5
    if x < 10 {
        print("")
    }
}

#[python]
fn add(n1 Int, n2 Int = 2) Int {

}


struct Rectangle(
    width Int,
    height Int
) 

impl Rectangle {

    fn (init) default() Rectangle {
        Rectangle(width=5, height=6)
    }

}

pub type IP

pub struct IPV6[IP] {
    
}


pub enum Color[
    Red #[string.repr="red"],
    Blue #[string.repr="blue"],
    Green #[string.repr="green"],
]

impl [Area, Perimeter] Rectangle {

    fn area() Int {
        return .width * .height
    }

    fn (owned) set_width(width Int) {
        .width = width
    }

}

Literals

1
1.0

"string"
["collection"]
palendrom = String#literal("Ada")
nums = Vec[Int]#literal()

impl Literal for Vec {

    fn literal() {
        
    }
}
```

`{}`: Code block

`()`: Collection block

```rust
x = Map[String, Int].new(
    ("Hello", 1),
    ("Hi", 2),
)

x = {
    "Hello": 1,
    "Hi": 2
}

struct TupleLiteral[T...] {

}

x RawUnion[InlineString, HeapAllocatedString]
x.as[InlineString]()


alias Shape = Union[Rectangle, Square, Circle]

alias Square = Rectangle

impl Shape {
    fn area(self Self) {
        if self.is<Rectangle>() {
            rect = self.as<Rectangle>()
            return rect.width * rect.height
        } else if self.is<Square>() {
            square = self.as<Square>()
            return square.side * square.side
        } else if self.is<Circle>() {
            circle = self.as<Circle>()
            return circle.radius * circle.radius * Circle.PI
        }
    }
}

#if y.is_type[Rectangle]() {
    print(y.width)
}

alias NoneType = ()
alias Optional[T] = Union[T, NoneType]

x Optional[Int] = None
if !x.is[NoneType]() {
    x = x.as[Int]()
    x += 1
}

// Sugar
x Int? = None
if x != None {
    x += 1
}

x Optional[Int] = Optional.none()
if !x.equals(None) {
    x = x.unwrap()
    x += 1
}

// More sugar
x = x? + 1

x = MyMap[String, Int].new(
    ("Hello", 1),
    ("Hi", 2),
)

#if x.[type] == MyMap[String, Int] {
    // Do something
}

#fn add(n1 Int, n2 Int) -> Int {
    return n1 + n2
}


pub struct Rectangle {
    #if env.Name == "gpu" {
        width GPUFloat
    }
}



```

```rust

n1.add(n2)
// math
trait Addition {
    fn add(self Self, other Self) Self
}

trait Arithmetic :: Addition, Subtraction, Multiplication, Division

trait Exists
/*
Complex explanation of expectations if you choose to implement this 
trait (not enforced by the compiler)
*/

// main

import math
import io

struct Int {
    ...
}

impl Int {

}

impl Int :: math.Arithmetic, io.Writable {

    fn add(self Self, other Self) Self {
        return self + other
    }
    ...

}

fn add[T comptime impls(T, math.Addition)](n1 T, n2 T) T {
    return n1 + n2
}


alias Type = ()

impl Type {

    #[comptime_only]
    fn implements(self Self, I Interface...) Bool {
        inline for interface in I.comptime_iter() {
            inline for function in interface.function_signatures() {
                if !self.has_signature(function) {
                    return false
                }
            }
        }
        return true
    }
}


fn safe_divide[T Type: T.implements(math.Division, comparison.Equals)](
    quotient T, 
    divisor T: divisor != 0) T {

}



type Person {
    type (Receptionist, Manager, Engineer)
}

type Optional[T] {
    type (Some(T), None)
}

impl Optional[T] {

    fn is_none(self Self) Bool {
        return self == None
    }

    fn unwrap(self Self) T {
        if self.is_none() {
            panic("Unwrap on None")
        }
        return self.as[Some(T)]()
    }

}

if person == Person.Receptionist {
    // Do something
} else if person == Person.Manager {
    // Do something else
} else if person == Person.Engineer {
    // Do something else
}

enum Color {
    Green,
    Blue
}

type IpAddr {
    type V4 {
        parts FixedArray[4; UInt8]
    },
    type V6 {
        value String
    },
}

impl IpAddr.V4 {

    fn route(self Self) {
        first = self.parts[0]
        ...
    }
}




```

## Magic Types

`Type`: This is the base of all things

```rust
fn implements(self Self, interfaces Interface...) Bool
fn callable(self Self, structure CallableStructure = AnyType) Bool
fn size(self Self) UInt64
fn fields(self Self) Field...
```

`Interface`: Description of interfaces
`Field`:

```
struct Field {
    T Type
    name: String
    attributes Map[String, AnyType]
}
```

base_functions: Functions


```rust


variant Color {
    red,
    green,
    blue,
}

impl Color {
    fn to_string(self Self) String {
        match self {
            red => "red",
            green => "green",
            blue => "blue"
        }
    }
}

struct Rectangle {
    width: Int
    height: Int
    color: Color
}

variant Shape {
    rectangle: Rectangle
    circle: struct {
        radius: Int
        color: Color
    }
}

shape = Shape {
    rectangle = Rectangle {
        width = 5, 
        height = 10, 
        color = Color {red}
    },
}

match shape {
    {rectangle: {width: 5}} if width < 6 => {
        print("Rectangle with width: {rectangle.width}")
    },
    {circle} => {
        print("Circle with radius: {circle.radius}")
    }
}

if shape:circle && shape.circle.radius > 0 {
    print("Circle with radius: ", shape.circle.radius)
} else if shape:rectangle {
    print("Rectangle with width: ", shape.rectangle.width)
}
```

```rust

variant Optional[T] {
    some: T
    null
}

impl VerifiedAutoInto[T] for Optional[T] {

    #disable_when(!self:some)
    fn auto_into(self Self) T {
        return self.some
    }
}


impl AutoFrom[T] for Optional[T] {

    fn auto_from(value T) Self {
        return Optional[T] {
            some = value
        }
    }

}

fn main() {
    x: Optional[Int64] = 5
    if !x:null {
        print("x is None")
    } else {
        print(`{x}`)
    }
}

struct InlineString {
    chars FixedArray[23; Uint8];
    length_remaining UInt8;
}

struct StringBuffer {
    ptr Ref[Array[UInt8]];
    length UInt64;
    capacity UInt64;
}

struct CowStringBuffer {
    ptr Ref[LargeStringHeap];
    _ UInt64;
    flag UInt64;
}

struct LargeStringHeap {
    ref_count Atomic[UInt64];
    chars Array[UInt8];
}


union String {
    inline InlineString,
    medium MediumString
    large LargeString
}

impl String {

    
}

```
