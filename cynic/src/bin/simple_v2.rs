fn main() {
    #[allow(unused_imports)]
    use queries::{TestArgs, TestStruct};

    //println!("{}", cynic::Operation::query<TestStruct>(TestArgs {}));
}

mod schema {
    cynic::use_schema!("src/bin/simple.graphql");
}

#[cynic::query_module(
    schema_path = "src/bin/simple.graphql",
    // TODO: Make query_module optional (automatically does it if missing)
    // Some complications around what to do with scalars here though...
    query_module = "schema"
)]
mod queries {
    use super::schema;

    #[derive(cynic::FragmentArguments)]
    pub struct TestArgs {}

    #[derive(cynic::QueryFragment)]
    #[cynic(argument_struct = "TestArgs")]
    pub struct TestStruct {
        #[arguments(x = 1, y = "1")]
        pub field_one: String,
        pub nested: Nested,
        pub opt_nested: Option<Nested>,
        pub dessert: Option<Dessert>,
    }

    #[derive(cynic::QueryFragment)]
    pub struct Nested {
        pub a_string: String,
        pub opt_string: Option<String>,
    }

    #[derive(cynic::QueryFragment)]
    #[cynic(graphql_type = "TestStruct")]
    pub struct Test {
        #[arguments(x = 1, y = "1")]
        pub field_one: String,
    }

    #[derive(cynic::Enum, Clone, Copy)]
    #[cynic(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Dessert {
        Cheesecake,
        IceCream,
    }

    #[derive(cynic::InlineFragments)]
    pub enum MyUnionType {
        TestStruct(Test),
        Nested(Nested),
    }
}

impl cynic::QueryRoot for schema::TestStruct {}
