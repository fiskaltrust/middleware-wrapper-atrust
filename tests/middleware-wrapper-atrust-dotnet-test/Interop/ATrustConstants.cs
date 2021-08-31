namespace fiskaltrust.MiddlewareWrapperAtrust.Test.Interop
{
    public static class ATrustConstants
    {
        public const string DllPath = "../../target/release/middleware_wrapper_atrust.dll";

        public static class TransactionType
        {
            public const string StartTransaction = "StartTransaction";
            public const string UpdateTransaction = "UpdateTransaction";
            public const string FinishTransaction = "FinishTransaction";
        }
    }
}
