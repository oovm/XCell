# UnityStorage Enum Conversion - Verification Checklist

- [x] UnityStorage is defined as an enum with variants for Binary, Json, Xml, and Protobuf
- [x] Each enum variant contains the corresponding configuration struct
- [x] UnityCodegen struct uses the new UnityStorage enum instead of the old struct
- [x] Serialization logic is updated to handle the new enum structure
- [x] Deserialization logic is updated to handle the new enum structure
- [x] Backward compatibility is maintained for existing configuration files
- [x] UnityCodegenHelper is updated to handle both old and new storage formats
- [x] All Unity-related tests pass (CocosStorage errors are outside scope)
- [x] New tests are created for the enum functionality
- [x] All storage format enum variants are tested
- [x] Edge cases and error handling are tested
- [x] The codebase remains clean and well-documented
- [x] Only one storage format can be selected at a time