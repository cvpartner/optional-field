use optional_field::Field::{self, *};
use postgres_types::{FromSql, IsNull, ToSql, Type};

mod from_sql {
    use super::*;

    #[test]
    fn decodes_value_as_present_some() {
        let raw = 42i32.to_be_bytes();

        let field = <Field<i32> as FromSql>::from_sql(&Type::INT4, &raw).unwrap();

        assert_eq!(Present(Some(42)), field);
    }

    #[test]
    fn decodes_null_as_present_none() {
        let field = <Field<i32> as FromSql>::from_sql_null(&Type::INT4).unwrap();

        assert_eq!(Present(None), field);
    }

    #[test]
    fn decodes_nullable_values() {
        let raw = 42i32.to_be_bytes();

        let value = <Field<i32> as FromSql>::from_sql_nullable(&Type::INT4, Some(&raw)).unwrap();
        let null = <Field<i32> as FromSql>::from_sql_nullable(&Type::INT4, None).unwrap();

        assert_eq!(Present(Some(42)), value);
        assert_eq!(Present(None), null);
    }

    #[test]
    fn accepts_inner_type() {
        assert!(<Field<i32> as FromSql>::accepts(&Type::INT4));
        assert!(!<Field<i32> as FromSql>::accepts(&Type::TEXT));
    }
}

mod to_sql {
    use super::*;

    #[test]
    fn encodes_present_some_as_value() {
        let field = Present(Some(42i32));
        let mut out = bytes::BytesMut::new();

        let result = field.to_sql(&Type::INT4, &mut out).unwrap();

        assert!(matches!(result, IsNull::No));
        assert_eq!(&42i32.to_be_bytes(), &out[..]);
    }

    #[test]
    fn encodes_present_none_as_null() {
        let field: Field<i32> = Present(None);
        let mut out = bytes::BytesMut::new();

        let result = field.to_sql(&Type::INT4, &mut out).unwrap();

        assert!(matches!(result, IsNull::Yes));
        assert!(out.is_empty());
    }

    #[test]
    fn encodes_missing_as_null() {
        let field: Field<i32> = Missing;
        let mut out = bytes::BytesMut::new();

        let result = field.to_sql(&Type::INT4, &mut out).unwrap();

        assert!(matches!(result, IsNull::Yes));
        assert!(out.is_empty());
    }

    #[test]
    fn accepts_inner_type() {
        assert!(<Field<i32> as ToSql>::accepts(&Type::INT4));
        assert!(!<Field<i32> as ToSql>::accepts(&Type::TEXT));
    }

    #[test]
    fn checked_encoding_rejects_wrong_type() {
        let field = Present(Some(42i32));
        let mut out = bytes::BytesMut::new();

        let result = field.to_sql_checked(&Type::TEXT, &mut out);

        assert!(result.is_err());
        assert!(out.is_empty());
    }
}
