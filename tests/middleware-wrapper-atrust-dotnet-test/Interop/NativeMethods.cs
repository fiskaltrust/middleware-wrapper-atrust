using System;
using System.Runtime.InteropServices;

namespace fiskaltrust.MiddlewareWrapperAtrust.Test.Interop
{
    internal class NativeMethods
    {
        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult startTransaction(byte[] clientId, uint clientIdLength, byte[] processData, uint processDataLength,
            byte[] processType, uint processTypeLength, byte[] additionalData, uint additionalDataLength, ref uint transactionNumber,
            ref long tm, ref IntPtr serialNumber, ref uint serialNumberLength, ref uint signatureCounter, ref IntPtr signatureValue,
            ref uint signatureValueLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult updateTransaction(byte[] clientId, uint clientIdLength, uint transactionNumber, byte[] processData,
            uint processDataLength, byte[] processType, uint processTypeLength, ref long tm, ref IntPtr signatureValue,
            ref uint signatureValueLength, ref uint signatureCounter);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult finishTransaction(byte[] clientId, uint clientIdLength, uint transactionNumber, byte[] processData,
            uint processDataLength, byte[] processType, uint processTypeLength, byte[] additionalData, uint additionalDataLength,
            ref long tm, ref IntPtr signatureValue, ref uint signatureValueLength, ref uint signatureCounter);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_registerClientId(byte[] clientId, uint clientIdLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult initializeDescriptionSet();

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult disableSecureElement();

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult authenticateUser(byte[] userId, uint userIdLength, byte[] pin, uint pinLength, ref uint authenticationResult, ref short remainingRetries);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult logOut(byte[] userId, uint userIdLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult exportData(uint maximumNumberRecords, ref IntPtr exportedData, ref uint exportedDataLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult exportDataFilteredByTransactionNumber(uint transactionNumber, ref IntPtr exportedData, ref uint exportedDataLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult exportCertificates(ref IntPtr certificates, ref uint certificatesLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult exportSerialNumbers(ref IntPtr serialNumbers, ref uint serialNumbersLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult getMaxNumberOfClients(ref uint maxNumberClients);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult getCurrentNumberOfClients(ref uint currentNumberClients);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult getMaxNumberOfTransactions(ref uint maxNumberTransactions);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult getCurrentNumberOfTransactions(ref uint currentNumberTransactions);

        [DllImport(ATrustConstants.DllPath)]
        public static extern void at_free(ref IntPtr ptr);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getLifecycleState(ref uint state);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getPublicKey(ref IntPtr publicKey, ref uint publicKeyLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getSerialNumber(ref IntPtr serialNumber, ref uint serialNumberLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getSignatureAlgorithm(ref IntPtr signatureAlgorithm, ref uint signatureAlgorithmLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getLogTimeFormat(ref IntPtr logTimeFormat, ref uint logTimeFormatLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getVersion(ref IntPtr version, ref uint versionLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getSignatureCounter(ref uint signatureCounter);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_getOpenTransactions(ref IntPtr transactionNumbers, ref uint transactionNumbersLength);

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_load();

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult at_unload();

        [DllImport(ATrustConstants.DllPath)]
        public static extern ATrustResult exportDataFilteredByTransactionNumberIntervalAndClientId(uint startTransactionNumber, uint endTransactionNumber,
                                                     byte[] clientId, uint clientIdLength, uint maximumNumberRecords, ref IntPtr exportedData, ref uint exportedDataLength);
    }
}
