package critter

// #include "critter.h"
import "C"

// Export functions from component:example/greeting
var component_example_greeting ExportsComponentExampleGreeting = nil
func SetExportsComponentExampleGreeting(i ExportsComponentExampleGreeting) {
  component_example_greeting = i
}
type ExportsComponentExampleGreeting interface {
  Hello(greeting string) string 
}
//export exports_component_example_greeting_hello
func ExportsComponentExampleGreetingHello(greeting *C.critter_string_t, ret *C.critter_string_t) {
  defer C.critter_string_free(greeting)
  var lift_greeting string
  lift_greeting = C.GoStringN(greeting.ptr, C.int(greeting.len))
  result := component_example_greeting.Hello(lift_greeting)
  var lower_result C.critter_string_t
  
  lower_result.ptr = C.CString(result)
  lower_result.len = C.size_t(len(result))
  *ret = lower_result
  
}
