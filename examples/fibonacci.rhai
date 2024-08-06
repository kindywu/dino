const TARGET = 8;

fn fib(n) {
    if n < 2 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

print(`Running Fibonacci(${TARGET}) x ...`);
print("Ready... Go!");

let result;
let now = timestamp();

result = fib(TARGET);

print(`Finished. Run time = ${now.elapsed} seconds.`);

print(`Fibonacci number #${TARGET} = ${result}`);