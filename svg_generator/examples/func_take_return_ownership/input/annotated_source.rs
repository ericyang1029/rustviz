
fn <tspan class="fn" data-hash="0" hash="6">take_and_return_ownership</tspan>(<tspan data-hash="2">some_string</tspan> : String) -> String {
    <tspan class="fn" data-hash="0" hash="7">println!</tspan>("{}", <tspan data-hash="2">some_string</tspan>);
    <tspan data-hash="2">some_string</tspan>
}
  
fn main() {
    let mut <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="5">String::from</tspan>("hello");
    <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="6">take_and_return_ownership</tspan>(<tspan data-hash="1">s</tspan>);
    <tspan class="fn" data-hash="0" hash="7">println!</tspan>("{}", <tspan data-hash="1">s</tspan>);   // OK
}