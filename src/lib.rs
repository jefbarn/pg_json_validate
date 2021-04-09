use pgx::*;
use jsonschema::JSONSchema;
use pgx::pg_sys::ERROR;
use pgx::pg_sys::{ErrorData, ThrowErrorData};
use pgx::PgSqlErrorCode::ERRCODE_DATA_EXCEPTION;
use std::ffi::CString;

pg_module_magic!();

#[pg_extern]
fn json_schema_is_valid(schema: JsonB, instance: JsonB) -> bool {
    jsonschema::is_valid(&schema.0, &instance.0)
}

#[pg_extern]
fn json_schema_validate(schema: JsonB, instance: JsonB) -> bool {
    let compiled = JSONSchema::compile(&schema.0).unwrap_or_else(|err| panic!("{}", err));

    let result = compiled.validate(&instance.0);
    if let Err(errors) = result {
        for error in errors {
            unsafe {
                ThrowErrorData(&mut ErrorData {
                    elevel: ERROR as i32,
                    sqlerrcode: ERRCODE_DATA_EXCEPTION as i32,
                    message: CString::new("JSON Schema validation error")
                        .unwrap()
                        .into_raw(),
                    detail: CString::new(error.to_string()).unwrap().into_raw(),
                    ..Default::default()
                });
            }
        }
        false
    } else {
        true
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_json_schema_validate() {
        let valid = Spi::get_one::<bool>(
            "select json_schema_validate('{\"maxLength\": 5}', '\"foobar\"'::jsonb)",
        );
        assert_eq!(valid, None)
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
