# UnityStorage Enum Conversion - Verification Checklist

- [ ] UnityStorage is defined as an enum with variants for Binary, Json, Xml, and Protobuf
- [ ] Each enum variant contains the corresponding configuration struct
- [ ] UnityCodegen struct uses the new UnityStorage enum instead of the old struct
- [ ] Serialization logic is updated to handle the new enum structure
- [ ] Deserialization logic is updated to handle the new enum structure
- [ ] Backward compatibility is maintained for existing configuration files
- [ ] UnityCodegenHelper is updated to handle both old and new storage formats
- [ ] All existing tests pass
- [ ] New tests are created for the enum functionality
- [ ] All storage format enum variants are tested
- [ ] Edge cases and error handling are tested
- [ ] The codebase remains clean and well-documented
- [ ] Only one storage format can be selected at a time