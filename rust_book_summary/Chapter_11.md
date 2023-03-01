In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that “Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.” That doesn’t mean we shouldn’t try to test as much as we can!

- Each test is run in a new thread.

- Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the *PartialEq* and *Debug* traits. All primitive types and most of the standard library types implement these traits. 

- For self-defined structs and enums implement *PartialEq* to assert equality of those types. And *Debug* to print the values when the assertion fails. Because both traits are derivable traits, add the #[derive(PartialEq, Debug)] annotation to the struct or enum definition.

