    [DataContract, Serializable]
    public enum {{CLASS_NAME}} : long
    {
        SSR = 1,
        SR = 2,
    }

    public static class {{CLASS_NAME}}Extension
    {
        public static string GetName(this ArchiveType self)
        {
            return DataTableManager.Instance.ArchiveTypeTable.GetElement(self)!.name;
        }
    }
