# Typhoon

A walking tree interpreter for the Typhoon programming language.

## Syntax

Typhoon is a dynamically-typed, interpreted language with a syntax similar to C and JavaScript. Here are some basic examples:

### Variables

```typhoon
var name = "Alice";
var age = 25;
```

### Control Flow

#### If-Else

```typhoon
if (age > 18) {
    print "You are an adult.";
} else {
    print "You are a minor.";
}
```

#### While Loop

```typhoon
var count = 0;
while (count < 5) {
    print count;
    count = count + 1;
}
```

### Functions

```typhoon
fun greet(name) {
    print "Hello, " + name + "!";
}

greet("Alice");
```

### Classes

```typhoon
class Person {
    init(name, age) {
        this.name = name;
        this.age = age;
    }

    sayHello() {
        print "Hello, my name is " + this.name;
    }
}

var alice = Person("Alice", 25);
alice.sayHello();
```

## Features

- Dynamically typed
- Object-oriented (single inheritance)
- Functions as first-class citizens
- Lexical scoping

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/typhoon.git
   cd typhoon
   ```

2. Build and run:

   ```sh
   cargo run 
   ```

## Usage

Run a Typhoon script with:

```sh
typhoon script.typhoon
```

Or enter the REPL mode:

```sh
typhoon
>
```

## Contributing

Feel free to open issues and pull requests!

## License

This project is licensed under the MIT License.
