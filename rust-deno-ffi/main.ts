const library = Deno.dlopen("./target/debug/librust_deno_ffi.dylib", {
  add: {
    parameters: ['f64', 'f64'],
    result: 'f64'
  },
  print_string: {
    parameters: ['buffer'],
    result: 'void'
  }
})

const result = library.symbols.add(10,45.5)
console.log({result})

const my_string = "Hello world. From javascript.\0"
const str_pointer = new TextEncoder().encode(my_string);
library.symbols.print_string(str_pointer);
