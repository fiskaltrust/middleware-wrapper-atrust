using System;
using System.Runtime.InteropServices;
using fiskaltrust.MiddlewareWrapperAtrust.Test.Interop;

namespace fiskaltrust.MiddlewareWrapperAtrust.Test
{
    class Program
    {
        public static string CharStarStarToB64String(ref IntPtr valuePointer, ulong length)
        {
            if (length > 0)
            {
                var arrayRes = new byte[length];
                Marshal.Copy(valuePointer, arrayRes, 0, (int) length);
                return Convert.ToBase64String(arrayRes);
            }
            return string.Empty;
        }

        static void Main()
        {
            var publicKey = Marshal.AllocCoTaskMem(1024);
            var publicKeyLength = 0u;

            var return_code = NativeMethods.at_getPublicKey(ref publicKey, ref publicKeyLength);
            Console.WriteLine(return_code);

            var result = CharStarStarToB64String(ref publicKey, publicKeyLength);
            Console.WriteLine(result);

            NativeMethods.at_free(ref publicKey);

            try {
                var result_after_free = CharStarStarToB64String(ref publicKey, publicKeyLength);
                Console.WriteLine(result_after_free);
            } catch(Exception e) {
                Console.WriteLine(e);
            }
        }
    }
}
