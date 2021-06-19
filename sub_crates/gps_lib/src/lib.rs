//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.2
//!

#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
    namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
    namespace = "xsd: http://www.w3.org/2001/XMLSchema",
    prefix = "soap"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}

pub mod messages {
    use super::*;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_UpdateBankingEnabledCardSoapIn")]
    pub struct WsBankingUpdateBankingEnabledCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingUpdateBankingEnabledCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_UpdateBankingEnabledCardResponse")]
    pub struct WsBankingUpdateBankingEnabledCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingUpdateBankingEnabledCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_UpdateBankingEnabledCardAuthSoapHeader")]
    pub struct WsBankingUpdateBankingEnabledCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_StatusQueryBankingEnabledCardSoapIn")]
    pub struct WsBankingStatusQueryBankingEnabledCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingStatusQueryBankingEnabledCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_StatusQueryBankingEnabledCardResponse")]
    pub struct WsBankingStatusQueryBankingEnabledCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingStatusQueryBankingEnabledCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_StatusQueryBankingEnabledCardAuthSoapHeader")]
    pub struct WsBankingStatusQueryBankingEnabledCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_CreateCustomerSoapIn")]
    pub struct WsBankingCreateCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingCreateCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_CreateCustomerResponse")]
    pub struct WsBankingCreateCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingCreateCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_CreateCustomerAuthSoapHeader")]
    pub struct WsBankingCreateCustomerAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_RegisterNotificationSoapIn")]
    pub struct WsBankingRegisterNotificationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingRegisterNotification,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_RegisterNotificationResponse")]
    pub struct WsBankingRegisterNotificationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingRegisterNotificationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_RegisterNotificationAuthSoapHeader")]
    pub struct WsBankingRegisterNotificationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_AccountModulusCheckSoapIn")]
    pub struct WsBankingAccountModulusCheckSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingAccountModulusCheck,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_AccountModulusCheckResponse")]
    pub struct WsBankingAccountModulusCheckSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingAccountModulusCheckResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_AccountModulusCheckAuthSoapHeader")]
    pub struct WsBankingAccountModulusCheckAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCardSoapIn")]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingGetDirectDebitInstructionsBankingEnabledCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCardResponse")]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingGetDirectDebitInstructionsBankingEnabledCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCardAuthSoapHeader")]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_CancelDirectDebitBankingEnabledCardSoapIn")]
    pub struct WsBankingCancelDirectDebitBankingEnabledCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingCancelDirectDebitBankingEnabledCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_CancelDirectDebitBankingEnabledCardResponse")]
    pub struct WsBankingCancelDirectDebitBankingEnabledCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingCancelDirectDebitBankingEnabledCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_CancelDirectDebitBankingEnabledCardAuthSoapHeader")]
    pub struct WsBankingCancelDirectDebitBankingEnabledCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_GetPendingDirectDebitsSoapIn")]
    pub struct WsBankingGetPendingDirectDebitsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingGetPendingDirectDebits,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_GetPendingDirectDebitsResponse")]
    pub struct WsBankingGetPendingDirectDebitsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingGetPendingDirectDebitsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_GetPendingDirectDebitsAuthSoapHeader")]
    pub struct WsBankingGetPendingDirectDebitsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_Card_Statement_V2SoapIn")]
    pub struct WsBankingCardStatementV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingCardStatementV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_Card_Statement_V2Response")]
    pub struct WsBankingCardStatementV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingCardStatementV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_Card_Statement_V2AuthSoapHeader")]
    pub struct WsBankingCardStatementV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Insert3DSecureDetailsSoapIn")]
    pub struct WsInsert3DSecureDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsInsert3DSecureDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Insert3DSecureDetailsResponse")]
    pub struct WsInsert3DSecureDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsInsert3DSecureDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Insert3DSecureDetailsAuthSoapHeader")]
    pub struct WsInsert3DSecureDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update3DSecureDetailsSoapIn")]
    pub struct WsUpdate3DSecureDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdate3DSecureDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update3DSecureDetailsResponse")]
    pub struct WsUpdate3DSecureDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdate3DSecureDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update3DSecureDetailsAuthSoapHeader")]
    pub struct WsUpdate3DSecureDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UpdateLastModifiedTypeSoapIn")]
    pub struct WsUpdateLastModifiedTypeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateLastModifiedType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UpdateLastModifiedTypeResponse")]
    pub struct WsUpdateLastModifiedTypeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateLastModifiedTypeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UpdateLastModifiedTypeAuthSoapHeader")]
    pub struct WsUpdateLastModifiedTypeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Delete3DSecureDetailsSoapIn")]
    pub struct WsDelete3DSecureDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsDelete3DSecureDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Delete3DSecureDetailsResponse")]
    pub struct WsDelete3DSecureDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsDelete3DSecureDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Delete3DSecureDetailsAuthSoapHeader")]
    pub struct WsDelete3DSecureDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Query3DSecureDetailsSoapIn")]
    pub struct WsQuery3DSecureDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsQuery3DSecureDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Query3DSecureDetailsResponse")]
    pub struct WsQuery3DSecureDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsQuery3DSecureDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Query3DSecureDetailsAuthSoapHeader")]
    pub struct WsQuery3DSecureDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GPS_Lock_UnlockSoapIn")]
    pub struct WsGPSLockUnlockSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGPSLockUnlock,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GPS_Lock_UnlockResponse")]
    pub struct WsGPSLockUnlockSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGPSLockUnlockResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GPS_Lock_UnlockAuthSoapHeader")]
    pub struct WsGPSLockUnlockAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WS_VerificationRequestSoapIn")]
    pub struct WsVerificationRequestSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsVerificationRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WS_VerificationRequestResponse")]
    pub struct WsVerificationRequestSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsVerificationRequestResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WS_VerificationRequestAuthSoapHeader")]
    pub struct WsVerificationRequestAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_AddUpDelCredentialsSoapIn")]
    pub struct WsAddUpDelCredentialsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsAddUpDelCredentials,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_AddUpDelCredentialsResponse")]
    pub struct WsAddUpDelCredentialsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsAddUpDelCredentialsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_AddUpDelCredentialsAuthSoapHeader")]
    pub struct WsAddUpDelCredentialsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_3DS_AddUpDelDetailsSoapIn")]
    pub struct Ws3DSAddUpDelDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::Ws3DSAddUpDelDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_3DS_AddUpDelDetailsResponse")]
    pub struct Ws3DSAddUpDelDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::Ws3DSAddUpDelDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_3DS_AddUpDelDetailsAuthSoapHeader")]
    pub struct Ws3DSAddUpDelDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceUpdateSoapIn")]
    pub struct WsBalanceUpdateSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceUpdate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceUpdateResponse")]
    pub struct WsBalanceUpdateSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceUpdateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceUpdateAuthSoapHeader")]
    pub struct WsBalanceUpdateAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Payment_Token_GetSoapIn")]
    pub struct WsPaymentTokenGetSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPaymentTokenGet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Payment_Token_GetResponse")]
    pub struct WsPaymentTokenGetSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPaymentTokenGetResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Payment_Token_GetAuthSoapHeader")]
    pub struct WsPaymentTokenGetAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Payment_Token_StatusChangeSoapIn")]
    pub struct WsPaymentTokenStatusChangeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPaymentTokenStatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Payment_Token_StatusChangeResponse")]
    pub struct WsPaymentTokenStatusChangeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPaymentTokenStatusChangeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Payment_Token_StatusChangeAuthSoapHeader")]
    pub struct WsPaymentTokenStatusChangeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ActivateSoapIn")]
    pub struct WsActivateSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsActivate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ActivateResponse")]
    pub struct WsActivateSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsActivateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ActivateAuthSoapHeader")]
    pub struct WsActivateAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_LoadSoapIn")]
    pub struct WsLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_LoadResponse")]
    pub struct WsLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_LoadAuthSoapHeader")]
    pub struct WsLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UnLoadSoapIn")]
    pub struct WsUnLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUnLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UnLoadResponse")]
    pub struct WsUnLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUnLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UnLoadAuthSoapHeader")]
    pub struct WsUnLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_StatusChangeSoapIn")]
    pub struct WsStatusChangeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsStatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_StatusChangeResponse")]
    pub struct WsStatusChangeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsStatusChangeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_StatusChangeAuthSoapHeader")]
    pub struct WsStatusChangeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_EnquirySoapIn")]
    pub struct WsEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_EnquiryResponse")]
    pub struct WsEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_EnquiryAuthSoapHeader")]
    pub struct WsEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceTransferSoapIn")]
    pub struct WsBalanceTransferSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceTransfer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceTransferResponse")]
    pub struct WsBalanceTransferSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceTransferResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceTransferAuthSoapHeader")]
    pub struct WsBalanceTransferAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_EnquirySoapIn")]
    pub struct WsBalanceEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_EnquiryResponse")]
    pub struct WsBalanceEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_EnquiryAuthSoapHeader")]
    pub struct WsBalanceEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_RepSoapIn")]
    pub struct WsBalanceEnquiryRepSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryRep,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_RepResponse")]
    pub struct WsBalanceEnquiryRepSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryRepResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_RepAuthSoapHeader")]
    pub struct WsBalanceEnquiryRepAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_V2SoapIn")]
    pub struct WsBalanceEnquiryV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_V2Response"
        namespace = "http://www.globalprocessing.ae/HyperionWeb",
        )]
    pub struct WsBalanceEnquiryV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_V2AuthSoapHeader")]
    pub struct WsBalanceEnquiryV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_WalletSoapIn")]
    pub struct WsBalanceEnquiryWalletSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryWallet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_WalletResponse")]
    pub struct WsBalanceEnquiryWalletSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceEnquiryWalletResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Balance_Enquiry_WalletAuthSoapHeader")]
    pub struct WsBalanceEnquiryWalletAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_StatementSoapIn")]
    pub struct WsCardStatementSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardStatement,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_StatementResponse")]
    pub struct WsCardStatementSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardStatementResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_StatementAuthSoapHeader")]
    pub struct WsCardStatementAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Statement_RepSoapIn")]
    pub struct WsCardStatementRepSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardStatementRep,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Statement_RepResponse")]
    pub struct WsCardStatementRepSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardStatementRepResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Statement_RepAuthSoapHeader")]
    pub struct WsCardStatementRepAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Customer_EnquirySoapIn")]
    pub struct WsCustomerEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCustomerEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Customer_EnquiryResponse")]
    pub struct WsCustomerEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCustomerEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Customer_EnquiryAuthSoapHeader")]
    pub struct WsCustomerEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Customer_Enquiry_V2SoapIn")]
    pub struct WsCustomerEnquiryV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCustomerEnquiryV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Customer_Enquiry_V2Response")]
    pub struct WsCustomerEnquiryV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCustomerEnquiryV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Customer_Enquiry_V2AuthSoapHeader")]
    pub struct WsCustomerEnquiryV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update_Cardholder_DetailsSoapIn")]
    pub struct WsUpdateCardholderDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateCardholderDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update_Cardholder_DetailsResponse")]
    pub struct WsUpdateCardholderDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateCardholderDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update_Cardholder_DetailsAuthSoapHeader")]
    pub struct WsUpdateCardholderDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UnLoad_StatusChangeSoapIn")]
    pub struct WsUnLoadStatusChangeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUnLoadStatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UnLoad_StatusChangeResponse")]
    pub struct WsUnLoadStatusChangeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUnLoadStatusChangeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UnLoad_StatusChangeAuthSoapHeader")]
    pub struct WsUnLoadStatusChangeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Activate_LoadSoapIn")]
    pub struct WsActivateLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsActivateLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Activate_LoadResponse")]
    pub struct WsActivateLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsActivateLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Activate_LoadAuthSoapHeader")]
    pub struct WsActivateLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceAdjustmentSoapIn")]
    pub struct WsBalanceAdjustmentSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceAdjustment,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceAdjustmentResponse")]
    pub struct WsBalanceAdjustmentSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBalanceAdjustmentResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BalanceAdjustmentAuthSoapHeader")]
    pub struct WsBalanceAdjustmentAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ExtendExpirySoapIn")]
    pub struct WsExtendExpirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsExtendExpiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ExtendExpiryResponse")]
    pub struct WsExtendExpirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsExtendExpiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ExtendExpiryAuthSoapHeader")]
    pub struct WsExtendExpiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Transaction_VoidSoapIn")]
    pub struct WsTransactionVoidSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsTransactionVoid,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Transaction_VoidResponse")]
    pub struct WsTransactionVoidSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsTransactionVoidResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Transaction_VoidAuthSoapHeader")]
    pub struct WsTransactionVoidAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardHolder_Details_EnquirySoapIn")]
    pub struct WsCardHolderDetailsEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardHolderDetailsEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardHolder_Details_EnquiryResponse")]
    pub struct WsCardHolderDetailsEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardHolderDetailsEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardHolder_Details_EnquiryAuthSoapHeader")]
    pub struct WsCardHolderDetailsEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardHolder_Details_Enquiry_V2SoapIn")]
    pub struct WsCardHolderDetailsEnquiryV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardHolderDetailsEnquiryV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardHolder_Details_Enquiry_V2Response")]
    pub struct WsCardHolderDetailsEnquiryV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardHolderDetailsEnquiryV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardHolder_Details_Enquiry_V2AuthSoapHeader")]
    pub struct WsCardHolderDetailsEnquiryV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Phone_ActivationSoapIn")]
    pub struct WsPhoneActivationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPhoneActivation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Phone_ActivationResponse")]
    pub struct WsPhoneActivationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPhoneActivationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Phone_ActivationAuthSoapHeader")]
    pub struct WsPhoneActivationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BulkCreationSoapIn")]
    pub struct WsBulkCreationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBulkCreation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BulkCreationResponse")]
    pub struct WsBulkCreationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBulkCreationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BulkCreationAuthSoapHeader")]
    pub struct WsBulkCreationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BulkWalletCreationSoapIn")]
    pub struct WsBulkWalletCreationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBulkWalletCreation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BulkWalletCreationResponse")]
    pub struct WsBulkWalletCreationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBulkWalletCreationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_BulkWalletCreationAuthSoapHeader")]
    pub struct WsBulkWalletCreationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_WebServiceResultSoapIn")]
    pub struct WsWebServiceResultSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsWebServiceResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_WebServiceResultResponse")]
    pub struct WsWebServiceResultSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsWebServiceResultResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_WebServiceResultAuthSoapHeader")]
    pub struct WsWebServiceResultAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Generic_FeesSoapIn")]
    pub struct WsGenericFeesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGenericFees,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Generic_FeesResponse")]
    pub struct WsGenericFeesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGenericFeesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Generic_FeesAuthSoapHeader")]
    pub struct WsGenericFeesAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_BalEnqSoapIn")]
    pub struct WsCardBalEnqSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardBalEnq,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_BalEnqResponse")]
    pub struct WsCardBalEnqSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardBalEnqResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_BalEnqAuthSoapHeader")]
    pub struct WsCardBalEnqAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WS_PinControlSoapIn")]
    pub struct WsPinControlSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPinControl,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WS_PinControlResponse")]
    pub struct WsPinControlSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPinControlResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WS_PinControlAuthSoapHeader")]
    pub struct WsPinControlAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateCardSoapIn")]
    pub struct WsCreateCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCreateCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateCardResponse")]
    pub struct WsCreateCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCreateCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateCardAuthSoapHeader")]
    pub struct WsCreateCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateWalletSoapIn")]
    pub struct WsCreateWalletSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCreateWallet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateWalletResponse")]
    pub struct WsCreateWalletSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCreateWalletResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateWalletAuthSoapHeader")]
    pub struct WsCreateWalletAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_RegenerateSoapIn")]
    pub struct WsRegenerateSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsRegenerate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_RegenerateResponse")]
    pub struct WsRegenerateSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsRegenerateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_RegenerateAuthSoapHeader")]
    pub struct WsRegenerateAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Convert_CardSoapIn")]
    pub struct WsConvertCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsConvertCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Convert_CardResponse")]
    pub struct WsConvertCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsConvertCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Convert_CardAuthSoapHeader")]
    pub struct WsConvertCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Change_GroupsSoapIn")]
    pub struct WsChangeGroupsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsChangeGroups,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Change_GroupsResponse")]
    pub struct WsChangeGroupsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsChangeGroupsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Change_GroupsAuthSoapHeader")]
    pub struct WsChangeGroupsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CheckSoapIn")]
    pub struct WsCheckSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCheck,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CheckResponse")]
    pub struct WsCheckSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCheckResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CheckAuthSoapHeader")]
    pub struct WsCheckAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Simple_CheckSoapIn")]
    pub struct WsSimpleCheckSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSimpleCheck,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Simple_CheckResponse")]
    pub struct WsSimpleCheckSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSimpleCheckResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Client_FxSoapIn")]
    pub struct WsClientFxSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsClientFx,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Client_FxResponse")]
    pub struct WsClientFxSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsClientFxResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Client_FxAuthSoapHeader")]
    pub struct WsClientFxAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Link_CardsSoapIn")]
    pub struct WsLinkCardsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsLinkCards,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Link_CardsResponse")]
    pub struct WsLinkCardsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsLinkCardsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Link_CardsAuthSoapHeader")]
    pub struct WsLinkCardsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_GroupSoapIn")]
    pub struct WsListGroupSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListGroup,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_GroupResponse")]
    pub struct WsListGroupSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListGroupResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_GroupAuthSoapHeader")]
    pub struct WsListGroupAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_ProductsSoapIn")]
    pub struct WsListProductsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListProducts,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_ProductsResponse")]
    pub struct WsListProductsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListProductsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_ProductsAuthSoapHeader")]
    pub struct WsListProductsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GetCardRequestSoapIn")]
    pub struct WsGetCardRequestSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetCardRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GetCardRequestResponse")]
    pub struct WsGetCardRequestSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetCardRequestResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GetCardRequestAuthSoapHeader")]
    pub struct WsGetCardRequestAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GetCardRequestStatusSoapIn")]
    pub struct WsGetCardRequestStatusSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetCardRequestStatus,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GetCardRequestStatusResponse")]
    pub struct WsGetCardRequestStatusSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetCardRequestStatusResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GetCardRequestStatusAuthSoapHeader")]
    pub struct WsGetCardRequestStatusAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardAcceptorWhiteListSoapIn")]
    pub struct WsCardAcceptorWhiteListSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardAcceptorWhiteList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardAcceptorWhiteListResponse")]
    pub struct WsCardAcceptorWhiteListSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardAcceptorWhiteListResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardAcceptorWhiteListAuthSoapHeader")]
    pub struct WsCardAcceptorWhiteListAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardAcceptorBlackListSoapIn")]
    pub struct WsCardAcceptorBlackListSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardAcceptorBlackList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardAcceptorBlackListResponse")]
    pub struct WsCardAcceptorBlackListSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardAcceptorBlackListResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CardAcceptorBlackListAuthSoapHeader")]
    pub struct WsCardAcceptorBlackListAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_SendMessageSoapIn")]
    pub struct WsSendMessageSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSendMessage,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_SendMessageResponse")]
    pub struct WsSendMessageSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSendMessageResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_SendMessageAuthSoapHeader")]
    pub struct WsSendMessageAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_MVCLoadSoapIn")]
    pub struct WsMVCLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsMVCLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_MVCLoadResponse")]
    pub struct WsMVCLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsMVCLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_MVCLoadAuthSoapHeader")]
    pub struct WsMVCLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Pending_FeesSoapIn")]
    pub struct WsListPendingFeesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListPendingFees,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Pending_FeesResponse")]
    pub struct WsListPendingFeesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListPendingFeesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Pending_FeesAuthSoapHeader")]
    pub struct WsListPendingFeesAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_WebServiceResult_V2SoapIn")]
    pub struct WsWebServiceResultV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsWebServiceResultV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_WebServiceResult_V2Response")]
    pub struct WsWebServiceResultV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsWebServiceResultV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_WebServiceResult_V2AuthSoapHeader")]
    pub struct WsWebServiceResultV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Get_PasscodeSoapIn")]
    pub struct WsGetPasscodeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetPasscode,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Get_PasscodeResponse")]
    pub struct WsGetPasscodeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetPasscodeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Get_PasscodeAuthSoapHeader")]
    pub struct WsGetPasscodeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Get_Card_ExpireSoonSoapIn")]
    pub struct WsGetCardExpireSoonSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetCardExpireSoon,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Get_Card_ExpireSoonResponse")]
    pub struct WsGetCardExpireSoonSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGetCardExpireSoonResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Get_Card_ExpireSoonAuthSoapHeader")]
    pub struct WsGetCardExpireSoonAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Send_CardFilesSoapIn")]
    pub struct WsSendCardFilesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSendCardFiles,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Send_CardFilesResponse")]
    pub struct WsSendCardFilesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSendCardFilesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Send_CardFilesAuthSoapHeader")]
    pub struct WsSendCardFilesAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_SafeReportsSoapIn")]
    pub struct WsSafeReportsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSafeReports,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_SafeReportsResponse")]
    pub struct WsSafeReportsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSafeReportsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_SafeReportsAuthSoapHeader")]
    pub struct WsSafeReportsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_RegenerateWalletSoapIn")]
    pub struct WsRegenerateWalletSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsRegenerateWallet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_RegenerateWalletResponse")]
    pub struct WsRegenerateWalletSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsRegenerateWalletResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_RegenerateWalletAuthSoapHeader")]
    pub struct WsRegenerateWalletAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UpdateLoadSourceSoapIn")]
    pub struct WsUpdateLoadSourceSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateLoadSource,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UpdateLoadSourceResponse")]
    pub struct WsUpdateLoadSourceSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateLoadSourceResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_UpdateLoadSourceAuthSoapHeader")]
    pub struct WsUpdateLoadSourceAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_MVCUnloadSoapIn")]
    pub struct WsMVCUnloadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsMVCUnload,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_MVCUnloadResponse")]
    pub struct WsMVCUnloadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsMVCUnloadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_MVCUnloadAuthSoapHeader")]
    pub struct WsMVCUnloadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Activate_MVCLoadSoapIn")]
    pub struct WsActivateMVCLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsActivateMVCLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Activate_MVCLoadResponse")]
    pub struct WsActivateMVCLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsActivateMVCLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Activate_MVCLoadAuthSoapHeader")]
    pub struct WsActivateMVCLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Renew_CardSoapIn")]
    pub struct WsRenewCardSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsRenewCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Renew_CardResponse")]
    pub struct WsRenewCardSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsRenewCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Renew_CardAuthSoapHeader")]
    pub struct WsRenewCardAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ResetAccumulatorSoapIn")]
    pub struct WsResetAccumulatorSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsResetAccumulator,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ResetAccumulatorResponse")]
    pub struct WsResetAccumulatorSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsResetAccumulatorResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_ResetAccumulatorAuthSoapHeader")]
    pub struct WsResetAccumulatorAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_EnrolSoapIn")]
    pub struct WsEnrolSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsEnrol,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_EnrolResponse")]
    pub struct WsEnrolSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsEnrolResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_EnrolAuthSoapHeader")]
    pub struct WsEnrolAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_ActivateSoapIn")]
    pub struct WsGiftCardActivateSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardActivate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_ActivateResponse")]
    pub struct WsGiftCardActivateSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardActivateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_ActivateAuthSoapHeader")]
    pub struct WsGiftCardActivateAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_LoadSoapIn")]
    pub struct WsGiftCardLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_LoadResponse")]
    pub struct WsGiftCardLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_LoadAuthSoapHeader")]
    pub struct WsGiftCardLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UnLoadSoapIn")]
    pub struct WsGiftCardUnLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUnLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UnLoadResponse")]
    pub struct WsGiftCardUnLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUnLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UnLoadAuthSoapHeader")]
    pub struct WsGiftCardUnLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_StatusChangeSoapIn")]
    pub struct WsGiftCardStatusChangeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardStatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_StatusChangeResponse")]
    pub struct WsGiftCardStatusChangeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardStatusChangeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_StatusChangeAuthSoapHeader")]
    pub struct WsGiftCardStatusChangeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_EnquirySoapIn")]
    pub struct WsGiftCardEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_EnquiryResponse")]
    pub struct WsGiftCardEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_EnquiryAuthSoapHeader")]
    pub struct WsGiftCardEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BalanceTransferSoapIn")]
    pub struct WsGiftCardBalanceTransferSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBalanceTransfer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BalanceTransferResponse")]
    pub struct WsGiftCardBalanceTransferSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBalanceTransferResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BalanceTransferAuthSoapHeader")]
    pub struct WsGiftCardBalanceTransferAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Balance_EnquirySoapIn")]
    pub struct WsGiftCardBalanceEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBalanceEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Balance_EnquiryResponse")]
    pub struct WsGiftCardBalanceEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBalanceEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Balance_EnquiryAuthSoapHeader")]
    pub struct WsGiftCardBalanceEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Card_StatementSoapIn")]
    pub struct WsGiftCardCardStatementSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardCardStatement,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Card_StatementResponse")]
    pub struct WsGiftCardCardStatementSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardCardStatementResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Card_StatementAuthSoapHeader")]
    pub struct WsGiftCardCardStatementAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Update_Cardholder_DetailsSoapIn")]
    pub struct WsGiftCardUpdateCardholderDetailsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUpdateCardholderDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Update_Cardholder_DetailsResponse")]
    pub struct WsGiftCardUpdateCardholderDetailsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUpdateCardholderDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Update_Cardholder_DetailsAuthSoapHeader")]
    pub struct WsGiftCardUpdateCardholderDetailsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UnLoad_StatusChangeSoapIn")]
    pub struct WsGiftCardUnLoadStatusChangeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUnLoadStatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UnLoad_StatusChangeResponse")]
    pub struct WsGiftCardUnLoadStatusChangeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUnLoadStatusChangeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UnLoad_StatusChangeAuthSoapHeader")]
    pub struct WsGiftCardUnLoadStatusChangeAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Activate_LoadSoapIn")]
    pub struct WsGiftCardActivateLoadSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardActivateLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Activate_LoadResponse")]
    pub struct WsGiftCardActivateLoadSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardActivateLoadResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Activate_LoadAuthSoapHeader")]
    pub struct WsGiftCardActivateLoadAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BalanceAdjustmentSoapIn")]
    pub struct WsGiftCardBalanceAdjustmentSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBalanceAdjustment,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BalanceAdjustmentResponse")]
    pub struct WsGiftCardBalanceAdjustmentSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBalanceAdjustmentResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BalanceAdjustmentAuthSoapHeader")]
    pub struct WsGiftCardBalanceAdjustmentAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_ExtendExpirySoapIn")]
    pub struct WsGiftCardExtendExpirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardExtendExpiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_ExtendExpiryResponse")]
    pub struct WsGiftCardExtendExpirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardExtendExpiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_ExtendExpiryAuthSoapHeader")]
    pub struct WsGiftCardExtendExpiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Transaction_VoidSoapIn")]
    pub struct WsGiftCardTransactionVoidSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardTransactionVoid,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Transaction_VoidResponse")]
    pub struct WsGiftCardTransactionVoidSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardTransactionVoidResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Transaction_VoidAuthSoapHeader")]
    pub struct WsGiftCardTransactionVoidAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_CardHolder_Details_EnquirySoapIn")]
    pub struct WsGiftCardCardHolderDetailsEnquirySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardCardHolderDetailsEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_CardHolder_Details_EnquiryResponse")]
    pub struct WsGiftCardCardHolderDetailsEnquirySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardCardHolderDetailsEnquiryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_CardHolder_Details_EnquiryAuthSoapHeader")]
    pub struct WsGiftCardCardHolderDetailsEnquiryAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Phone_ActivationSoapIn")]
    pub struct WsGiftCardPhoneActivationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardPhoneActivation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Phone_ActivationResponse")]
    pub struct WsGiftCardPhoneActivationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardPhoneActivationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Phone_ActivationAuthSoapHeader")]
    pub struct WsGiftCardPhoneActivationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BulkCreationSoapIn")]
    pub struct WsGiftCardBulkCreationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBulkCreation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BulkCreationResponse")]
    pub struct WsGiftCardBulkCreationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardBulkCreationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_BulkCreationAuthSoapHeader")]
    pub struct WsGiftCardBulkCreationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_WebServiceResultSoapIn")]
    pub struct WsGiftCardWebServiceResultSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardWebServiceResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_WebServiceResultResponse")]
    pub struct WsGiftCardWebServiceResultSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardWebServiceResultResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_WebServiceResultAuthSoapHeader")]
    pub struct WsGiftCardWebServiceResultAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Generic_FeesSoapIn")]
    pub struct WsGiftCardGenericFeesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardGenericFees,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Generic_FeesResponse")]
    pub struct WsGiftCardGenericFeesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardGenericFeesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Generic_FeesAuthSoapHeader")]
    pub struct WsGiftCardGenericFeesAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_PinControlSoapIn")]
    pub struct WsGiftCardPinControlSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardPinControl,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_PinControlResponse")]
    pub struct WsGiftCardPinControlSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardPinControlResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_PinControlAuthSoapHeader")]
    pub struct WsGiftCardPinControlAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UpdateLoadSourceSoapIn")]
    pub struct WsGiftCardUpdateLoadSourceSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUpdateLoadSource,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UpdateLoadSourceResponse")]
    pub struct WsGiftCardUpdateLoadSourceSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardUpdateLoadSourceResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_UpdateLoadSourceAuthSoapHeader")]
    pub struct WsGiftCardUpdateLoadSourceAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Activate_Load_ProductTpye_CPSoapIn")]
    pub struct WsGiftCardActivateLoadProductTpyeCPSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardActivateLoadProductTpyeCP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Activate_Load_ProductTpye_CPResponse")]
    pub struct WsGiftCardActivateLoadProductTpyeCPSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsGiftCardActivateLoadProductTpyeCPResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_GiftCard_Activate_Load_ProductTpye_CPAuthSoapHeader")]
    pub struct WsGiftCardActivateLoadProductTpyeCPAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_TransactionXMLSoapIn")]
    pub struct WsCardTransactionXMLSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardTransactionXML,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_TransactionXMLResponse")]
    pub struct WsCardTransactionXMLSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardTransactionXMLResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_TransactionXMLAuthSoapHeader")]
    pub struct WsCardTransactionXMLAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Change_GroupsSoapIn")]
    pub struct WsCardChangeGroupsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardChangeGroups,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Change_GroupsResponse")]
    pub struct WsCardChangeGroupsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardChangeGroupsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Change_GroupsAuthSoapHeader")]
    pub struct WsCardChangeGroupsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Change_Cardacceptor_ListSoapIn")]
    pub struct WsCardChangeCardacceptorListSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardChangeCardacceptorList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Change_Cardacceptor_ListResponse")]
    pub struct WsCardChangeCardacceptorListSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCardChangeCardacceptorListResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Card_Change_Cardacceptor_ListAuthSoapHeader")]
    pub struct WsCardChangeCardacceptorListAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Change_Cardacceptor_ListSoapIn")]
    pub struct WsChangeCardacceptorListSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsChangeCardacceptorList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Change_Cardacceptor_ListResponse")]
    pub struct WsChangeCardacceptorListSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsChangeCardacceptorListResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Change_Cardacceptor_ListAuthSoapHeader")]
    pub struct WsChangeCardacceptorListAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_AddressMatchCheckingSoapIn")]
    pub struct WsAddressMatchCheckingSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsAddressMatchChecking,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_AddressMatchCheckingResponse")]
    pub struct WsAddressMatchCheckingSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsAddressMatchCheckingResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_AddressMatchCheckingAuthSoapHeader")]
    pub struct WsAddressMatchCheckingAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_LicenseVerificationSoapIn")]
    pub struct WsLicenseVerificationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsLicenseVerification,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_LicenseVerificationResponse")]
    pub struct WsLicenseVerificationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsLicenseVerificationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_LicenseVerificationAuthSoapHeader")]
    pub struct WsLicenseVerificationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_PassportVerificationSoapIn")]
    pub struct WsPassportVerificationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPassportVerification,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_PassportVerificationResponse")]
    pub struct WsPassportVerificationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsPassportVerificationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_PassportVerificationAuthSoapHeader")]
    pub struct WsPassportVerificationAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Sanctions_PEP_CheckSoapIn")]
    pub struct WsSanctionsPEPCheckSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSanctionsPEPCheck,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Sanctions_PEP_CheckResponse")]
    pub struct WsSanctionsPEPCheckSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSanctionsPEPCheckResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Sanctions_PEP_CheckAuthSoapHeader")]
    pub struct WsSanctionsPEPCheckAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Sanctions_PEP_Check_V2SoapIn")]
    pub struct WsSanctionsPEPCheckV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSanctionsPEPCheckV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Sanctions_PEP_Check_V2Response")]
    pub struct WsSanctionsPEPCheckV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsSanctionsPEPCheckV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Sanctions_PEP_Check_V2AuthSoapHeader")]
    pub struct WsSanctionsPEPCheckV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Sanctions_PEPSoapIn")]
    pub struct WsListSanctionsPEPSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListSanctionsPEP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Sanctions_PEPResponse")]
    pub struct WsListSanctionsPEPSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListSanctionsPEPResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Sanctions_PEPAuthSoapHeader")]
    pub struct WsListSanctionsPEPAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Sanctions_PEP_MatchesSoapIn")]
    pub struct WsListSanctionsPEPMatchesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListSanctionsPEPMatches,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Sanctions_PEP_MatchesResponse")]
    pub struct WsListSanctionsPEPMatchesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsListSanctionsPEPMatchesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_List_Sanctions_PEP_MatchesAuthSoapHeader")]
    pub struct WsListSanctionsPEPMatchesAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update_Sanctions_PEP_MatchesSoapIn")]
    pub struct WsUpdateSanctionsPEPMatchesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateSanctionsPEPMatches,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update_Sanctions_PEP_MatchesResponse")]
    pub struct WsUpdateSanctionsPEPMatchesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsUpdateSanctionsPEPMatchesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Update_Sanctions_PEP_MatchesAuthSoapHeader")]
    pub struct WsUpdateSanctionsPEPMatchesAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateCard_V2SoapIn")]
    pub struct WsCreateCardV2SoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCreateCardV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateCard_V2Response")]
    pub struct WsCreateCardV2SoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsCreateCardV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_CreateCard_V2AuthSoapHeader")]
    pub struct WsCreateCardV2AuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_ReturnBankDetailsFromTokenSoapIn")]
    pub struct WsBankingReturnBankDetailsFromTokenSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingReturnBankDetailsFromToken,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_ReturnBankDetailsFromTokenResponse")]
    pub struct WsBankingReturnBankDetailsFromTokenSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingReturnBankDetailsFromTokenResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_ReturnBankDetailsFromTokenAuthSoapHeader")]
    pub struct WsBankingReturnBankDetailsFromTokenAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_ChangeAccountBankingFeaturesStatusSoapIn")]
    pub struct WsBankingChangeAccountBankingFeaturesStatusSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingChangeAccountBankingFeaturesStatus,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_ChangeAccountBankingFeaturesStatusResponse")]
    pub struct WsBankingChangeAccountBankingFeaturesStatusSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingChangeAccountBankingFeaturesStatusResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_ChangeAccountBankingFeaturesStatusAuthSoapHeader")]
    pub struct WsBankingChangeAccountBankingFeaturesStatusAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_TransferFundsSoapIn")]
    pub struct WsBankingTransferFundsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingTransferFunds,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_TransferFundsResponse")]
    pub struct WsBankingTransferFundsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::WsBankingTransferFundsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Ws_Banking_TransferFundsAuthSoapHeader")]
    pub struct WsBankingTransferFundsAuthSoapHeader {
        #[yaserde(flatten, default)]
        pub auth_soap_header: types::AuthSoapHeader,
    }
}

pub mod types {
    use super::*;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_UpdateBankingEnabledCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingUpdateBankingEnabledCard {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: UpdateAccountStatusRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateAccountStatusRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct UpdateAccountStatusRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "NewStatus", prefix = "tns", default)]
        pub new_status: EnumAccountStatus,
        #[yaserde(rename = "UpdateSubAccountsToSame", prefix = "tns", default)]
        pub update_sub_accounts_to_same: bool,
        #[yaserde(rename = "Notes", prefix = "tns", default)]
        pub notes: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_AccountStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumAccountStatus {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_UpdateBankingEnabledCardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingUpdateBankingEnabledCardResponse {
        #[yaserde(
            rename = "Ws_Banking_UpdateBankingEnabledCardResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_update_banking_enabled_card_result: UpdateAccountStatusResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateAccountStatusResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct UpdateAccountStatusResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i64,
        #[yaserde(rename = "Accounts", prefix = "tns", default)]
        pub accounts: Option<ArrayOfBLAccount>,
        #[yaserde(rename = "TotalBalance", prefix = "tns", default)]
        pub total_balance: f64,
        #[yaserde(rename = "CloseAccountStatus", prefix = "tns", default)]
        pub close_account_status: EnumCloseAccountStatus,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfBLAccount",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfBLAccount {
        #[yaserde(rename = "BLAccount", prefix = "tns", default)]
        pub bl_account: Vec<Blaccount>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BLAccount",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Blaccount {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i64,
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "AccountNumber", prefix = "tns", default)]
        pub account_number: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: EnumAccountStatus,
        #[yaserde(rename = "Balance", prefix = "tns", default)]
        pub balance: f64,
        #[yaserde(rename = "UpdateStatusResult", prefix = "tns", default)]
        pub update_status_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_CloseAccountStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumCloseAccountStatus {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AuthSoapHeader",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AuthSoapHeader {
        #[yaserde(rename = "strUserName", prefix = "tns", default)]
        pub str_user_name: Option<String>,
        #[yaserde(rename = "strPassword", prefix = "tns", default)]
        pub str_password: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_StatusQueryBankingEnabledCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingStatusQueryBankingEnabledCard {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: QueryAccountStatusRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "QueryAccountStatusRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct QueryAccountStatusRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_StatusQueryBankingEnabledCardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingStatusQueryBankingEnabledCardResponse {
        #[yaserde(
            rename = "Ws_Banking_StatusQueryBankingEnabledCardResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_status_query_banking_enabled_card_result: QueryAccountStatusResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "QueryAccountStatusResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct QueryAccountStatusResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "AccountStatus", prefix = "tns", default)]
        pub account_status: AccountStatus,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AccountStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AccountStatus {
        #[yaserde(rename = "AccountStatus", prefix = "tns", default)]
        pub account_status: EnumAccountStatus,
        #[yaserde(rename = "BankingInEnabled", prefix = "tns", default)]
        pub banking_in_enabled: EnumBankingInEnabled,
        #[yaserde(rename = "BankingOutEnabled", prefix = "tns", default)]
        pub banking_out_enabled: EnumBankingOutEnabled,
        #[yaserde(rename = "DirectDebitInEnabled", prefix = "tns", default)]
        pub direct_debit_in_enabled: EnumDirectDebitInEnabled,
        #[yaserde(rename = "DirectDebitOutEnabled", prefix = "tns", default)]
        pub direct_debit_out_enabled: EnumDirectDebitOutEnabled,
        #[yaserde(rename = "CardEnabled", prefix = "tns", default)]
        pub card_enabled: EnumCardEnabled,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_BankingInEnabled",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumBankingInEnabled {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_BankingOutEnabled",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumBankingOutEnabled {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitInEnabled",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitInEnabled {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitOutEnabled",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitOutEnabled {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_CardEnabled",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumCardEnabled {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_CreateCustomer",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCreateCustomer {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: Option<CreateCustomerRequest>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateCustomerRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CreateCustomerRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "Associates", prefix = "tns", default)]
        pub associates: Option<ArrayOfAssociate>,
        #[yaserde(rename = "DocumentInfo", prefix = "tns", default)]
        pub document_info: Option<ArrayOfDocumentInfo>,
        #[yaserde(rename = "ExpectedMonthlySpend", prefix = "tns", default)]
        pub expected_monthly_spend: f64,
        #[yaserde(rename = "ExternalReference", prefix = "tns", default)]
        pub external_reference: Option<String>,
        #[yaserde(rename = "IndustryCode", prefix = "tns", default)]
        pub industry_code: Option<String>,
        #[yaserde(rename = "RegisteredAddress", prefix = "tns", default)]
        pub registered_address: Option<BankingAddress>,
        #[yaserde(rename = "TCSVersion", prefix = "tns", default)]
        pub tcs_version: i32,
        #[yaserde(rename = "TradingAddress", prefix = "tns", default)]
        pub trading_address: Option<BankingAddress>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: CustomerType,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "CompanyRegNumber", prefix = "tns", default)]
        pub company_reg_number: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAssociate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfAssociate {
        #[yaserde(rename = "Associate", prefix = "tns", default)]
        pub associate: Vec<Associate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Associate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Associate {
        #[yaserde(rename = "applicant", prefix = "tns", default)]
        pub applicant: bool,
        #[yaserde(rename = "dateOfBirth", prefix = "tns", default)]
        pub date_of_birth: String,
        #[yaserde(rename = "documentInfo", prefix = "tns", default)]
        pub document_info: Option<ArrayOfDocumentInfo>,
        #[yaserde(rename = "email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "firstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "middleName", prefix = "tns", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "lastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "ownership", prefix = "tns", default)]
        pub ownership: i32,
        #[yaserde(rename = "phone", prefix = "tns", default)]
        pub phone: Option<String>,
        #[yaserde(rename = "type", prefix = "tns", default)]
        pub rs_type: Option<String>,
        #[yaserde(rename = "homeAddress", prefix = "tns", default)]
        pub home_address: Option<BankingAddress>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfDocumentInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfDocumentInfo {
        #[yaserde(rename = "DocumentInfo", prefix = "tns", default)]
        pub document_info: Vec<DocumentInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DocumentInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct DocumentInfo {
        #[yaserde(rename = "filename", prefix = "tns", default)]
        pub filename: Option<String>,
        #[yaserde(rename = "filepath", prefix = "tns", default)]
        pub filepath: Option<String>,
        #[yaserde(rename = "uploadDate", prefix = "tns", default)]
        pub upload_date: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BankingAddress",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BankingAddress {
        #[yaserde(rename = "addressLine1", prefix = "tns", default)]
        pub address_line_1: Option<String>,
        #[yaserde(rename = "addressLine2", prefix = "tns", default)]
        pub address_line_2: Option<String>,
        #[yaserde(rename = "posttown", prefix = "tns", default)]
        pub posttown: Option<String>,
        #[yaserde(rename = "postCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "country", prefix = "tns", default)]
        pub country: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerType",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CustomerType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_CreateCustomerResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCreateCustomerResponse {
        #[yaserde(rename = "Ws_Banking_CreateCustomerResult", prefix = "tns", default)]
        pub ws_banking_create_customer_result: CreateCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateCustomerResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CreateCustomerResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "CustomerID", prefix = "tns", default)]
        pub customer_id: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "Messages", prefix = "tns", default)]
        pub messages: Option<ArrayOfBankingError>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfBankingError",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfBankingError {
        #[yaserde(rename = "BankingError", prefix = "tns", default)]
        pub banking_error: Vec<BankingError>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BankingError",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BankingError {
        #[yaserde(rename = "field", prefix = "tns", default)]
        pub field: Option<String>,
        #[yaserde(rename = "code", prefix = "tns", default)]
        pub code: Option<String>,
        #[yaserde(rename = "message", prefix = "tns", default)]
        pub message: Option<String>,
        #[yaserde(rename = "error", prefix = "tns", default)]
        pub error: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_RegisterNotification",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingRegisterNotification {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: CreateNotificationRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateNotificationRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CreateNotificationRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: i32,
        #[yaserde(rename = "CustomerID", prefix = "tns", default)]
        pub customer_id: Option<String>,
        #[yaserde(rename = "channel", prefix = "tns", default)]
        pub channel: EnumChannel,
        #[yaserde(rename = "daysToRun", prefix = "tns", default)]
        pub days_to_run: Option<ArrayOfString>,
        #[yaserde(rename = "threshold", prefix = "tns", default)]
        pub threshold: f64,
        #[yaserde(rename = "timesToRun", prefix = "tns", default)]
        pub times_to_run: Option<ArrayOfString>,
        #[yaserde(rename = "destinations", prefix = "tns", default)]
        pub destinations: Option<ArrayOfString>,
        #[yaserde(rename = "type", prefix = "tns", default)]
        pub rs_type: EnumRequestType,
        #[yaserde(rename = "status", prefix = "tns", default)]
        pub status: EnumRequestStatus,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_Channel",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumChannel {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfString",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfString {
        #[yaserde(rename = "string", prefix = "tns", default)]
        pub string: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Enum_RequestType",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumRequestType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Enum_RequestStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumRequestStatus {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_RegisterNotificationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingRegisterNotificationResponse {
        #[yaserde(
            rename = "Ws_Banking_RegisterNotificationResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_register_notification_result: CreateNotificationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateNotificationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CreateNotificationResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Messages", prefix = "tns", default)]
        pub messages: Option<ArrayOfBankingError>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_AccountModulusCheck",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingAccountModulusCheck {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: AccountModulusCheckRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AccountModulusCheckRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AccountModulusCheckRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "AccountNumber", prefix = "tns", default)]
        pub account_number: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_AccountModulusCheckResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingAccountModulusCheckResponse {
        #[yaserde(
            rename = "Ws_Banking_AccountModulusCheckResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_account_modulus_check_result: AccountModulusCheckResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AccountModulusCheckResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AccountModulusCheckResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "VSeriesResponse", prefix = "tns", default)]
        pub v_series_response: VseriesAccountModulusCheckResponse,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "VseriesAccountModulusCheckResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct VseriesAccountModulusCheckResponse {
        #[yaserde(rename = "AccountDetailsInput", prefix = "tns", default)]
        pub account_details_input: AccountDetails,
        #[yaserde(rename = "AccountDetailsOutput", prefix = "tns", default)]
        pub account_details_output: AccountDetails,
        #[yaserde(rename = "AccountIBAN", prefix = "tns", default)]
        pub account_iban: Option<String>,
        #[yaserde(rename = "AccountTranscribed", prefix = "tns", default)]
        pub account_transcribed: bool,
        #[yaserde(rename = "UkBankBranch", prefix = "tns", default)]
        pub uk_bank_branch: UkBankBranch,
        #[yaserde(rename = "RequiresBuildingSocietyRollNumber", prefix = "tns", default)]
        pub requires_building_society_roll_number: bool,
        #[yaserde(
            rename = "BuildingSocietyRollNumberTranscribed",
            prefix = "tns",
            default
        )]
        pub building_society_roll_number_transcribed: bool,
        #[yaserde(rename = "PublicInvalidIssue", prefix = "tns", default)]
        pub public_invalid_issue: i32,
        #[yaserde(rename = "Valid", prefix = "tns", default)]
        pub valid: bool,
        #[yaserde(rename = "InvalidReason", prefix = "tns", default)]
        pub invalid_reason: Option<String>,
        #[yaserde(rename = "InvalidParameter", prefix = "tns", default)]
        pub invalid_parameter: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AccountDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AccountDetails {
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "AccountNumber", prefix = "tns", default)]
        pub account_number: Option<String>,
        #[yaserde(rename = "BuildingSocietyRollNumber", prefix = "tns", default)]
        pub building_society_roll_number: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UkBankBranch",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct UkBankBranch {
        #[yaserde(rename = "BankBIC", prefix = "tns", default)]
        pub bank_bic: Option<String>,
        #[yaserde(rename = "BranchBIC", prefix = "tns", default)]
        pub branch_bic: Option<String>,
        #[yaserde(rename = "BankName", prefix = "tns", default)]
        pub bank_name: Option<String>,
        #[yaserde(rename = "BankOfficeTitle", prefix = "tns", default)]
        pub bank_office_title: Option<String>,
        #[yaserde(rename = "BranchName", prefix = "tns", default)]
        pub branch_name: Option<String>,
        #[yaserde(rename = "OfficeTitle", prefix = "tns", default)]
        pub office_title: Option<String>,
        #[yaserde(rename = "ContactAddress1", prefix = "tns", default)]
        pub contact_address_1: Option<String>,
        #[yaserde(rename = "ContactAddress2", prefix = "tns", default)]
        pub contact_address_2: Option<String>,
        #[yaserde(rename = "ContactAddress3", prefix = "tns", default)]
        pub contact_address_3: Option<String>,
        #[yaserde(rename = "ContactAddress4", prefix = "tns", default)]
        pub contact_address_4: Option<String>,
        #[yaserde(rename = "ContactAddressCity", prefix = "tns", default)]
        pub contact_address_city: Option<String>,
        #[yaserde(rename = "ContactAddressCounty", prefix = "tns", default)]
        pub contact_address_county: Option<String>,
        #[yaserde(rename = "ContactAddressPostCode", prefix = "tns", default)]
        pub contact_address_post_code: Option<String>,
        #[yaserde(rename = "ContactAddressPostCountry", prefix = "tns", default)]
        pub contact_address_post_country: Option<String>,
        #[yaserde(rename = "ContactPhoneNumber", prefix = "tns", default)]
        pub contact_phone_number: Option<String>,
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "DateLastChanged", prefix = "tns", default)]
        pub date_last_changed: Option<String>,
        #[yaserde(rename = "TransactionInfo", prefix = "tns", default)]
        pub transaction_info: TransactionInfo,
        #[yaserde(rename = "ChapsBankBIC", prefix = "tns", default)]
        pub chaps_bank_bic: Option<String>,
        #[yaserde(rename = "ChapsBranchBIC", prefix = "tns", default)]
        pub chaps_branch_bic: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransactionInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct TransactionInfo {
        #[yaserde(rename = "BacsCredits", prefix = "tns", default)]
        pub bacs_credits: bool,
        #[yaserde(rename = "BacsDebits", prefix = "tns", default)]
        pub bacs_debits: bool,
        #[yaserde(rename = "FasterPaymentsService", prefix = "tns", default)]
        pub faster_payments_service: bool,
        #[yaserde(rename = "ChapsSterling", prefix = "tns", default)]
        pub chaps_sterling: bool,
        #[yaserde(rename = "DirectDebitInstructions", prefix = "tns", default)]
        pub direct_debit_instructions: bool,
        #[yaserde(rename = "UnpaidChequeClaims", prefix = "tns", default)]
        pub unpaid_cheque_claims: bool,
        #[yaserde(rename = "DividendInterest", prefix = "tns", default)]
        pub dividend_interest: bool,
        #[yaserde(rename = "BuildingSocietyInterest", prefix = "tns", default)]
        pub building_society_interest: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCard {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: GetDirectDebitInstructionsRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetDirectDebitInstructionsRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GetDirectDebitInstructionsRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<i32>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<i32>,
        #[yaserde(rename = "IncludeSubAccounts", prefix = "tns", default)]
        pub include_sub_accounts: bool,
        #[yaserde(rename = "CreatedFrom", prefix = "tns", default)]
        pub created_from: Option<String>,
        #[yaserde(rename = "DirectDebitStatus", prefix = "tns", default)]
        pub direct_debit_status: Option<EnumDirectDebitStatus>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitStatus {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCardResponse {
        #[yaserde(
            rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCardResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_get_direct_debit_instructions_banking_enabled_card_result:
            GetDirectDebitInstructionsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetDirectDebitInstructionsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GetDirectDebitInstructionsResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "DirectDebits", prefix = "tns", default)]
        pub direct_debits: Option<ArrayOfBLDirectDebit>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfBLDirectDebit",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfBLDirectDebit {
        #[yaserde(rename = "BLDirectDebit", prefix = "tns", default)]
        pub bl_direct_debit: Vec<BldirectDebit>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BLDirectDebit",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BldirectDebit {
        #[yaserde(rename = "Created", prefix = "tns", default)]
        pub created: String,
        #[yaserde(rename = "CreationMethod", prefix = "tns", default)]
        pub creation_method: EnumCreationMethod,
        #[yaserde(rename = "CreditorAccountName", prefix = "tns", default)]
        pub creditor_account_name: Option<String>,
        #[yaserde(rename = "CreditorBIC", prefix = "tns", default)]
        pub creditor_bic: Option<String>,
        #[yaserde(rename = "CreditorIBAN", prefix = "tns", default)]
        pub creditor_iban: Option<String>,
        #[yaserde(rename = "CreditorReference", prefix = "tns", default)]
        pub creditor_reference: Option<String>,
        #[yaserde(rename = "DebtorAccountID", prefix = "tns", default)]
        pub debtor_account_id: Option<String>,
        #[yaserde(rename = "DerivedReference", prefix = "tns", default)]
        pub derived_reference: Option<String>,
        #[yaserde(rename = "IBAN", prefix = "tns", default)]
        pub iban: Option<String>,
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Guid,
        #[yaserde(rename = "IgnoreTransactionCode", prefix = "tns", default)]
        pub ignore_transaction_code: bool,
        #[yaserde(rename = "IsInstructionHeldAtBank", prefix = "tns", default)]
        pub is_instruction_held_at_bank: bool,
        #[yaserde(rename = "IsPaperless", prefix = "tns", default)]
        pub is_paperless: bool,
        #[yaserde(rename = "IsThroughNotificationService", prefix = "tns", default)]
        pub is_through_notification_service: bool,
        #[yaserde(rename = "LastChanged", prefix = "tns", default)]
        pub last_changed: String,
        #[yaserde(rename = "Reference", prefix = "tns", default)]
        pub reference: Option<String>,
        #[yaserde(rename = "ServiceUserNumber", prefix = "tns", default)]
        pub service_user_number: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: EnumDirectDebitStatus,
        #[yaserde(rename = "SuppressFirstDirectDebit", prefix = "tns", default)]
        pub suppress_first_direct_debit: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_CreationMethod",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumCreationMethod {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_CancelDirectDebitBankingEnabledCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCancelDirectDebitBankingEnabledCard {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: CancelDirectDebitRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CancelDirectDebitRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CancelDirectDebitRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "DDIdentifier", prefix = "tns", default)]
        pub dd_identifier: Guid,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: EnumDirectDebitCancellationReason,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitCancellationReason",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitCancellationReason {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_CancelDirectDebitBankingEnabledCardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCancelDirectDebitBankingEnabledCardResponse {
        #[yaserde(
            rename = "Ws_Banking_CancelDirectDebitBankingEnabledCardResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_cancel_direct_debit_banking_enabled_card_result: CancelDirectDebitResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CancelDirectDebitResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CancelDirectDebitResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "Response", prefix = "tns", default)]
        pub response: EnumDirectDebitCancelStatus,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitCancelStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitCancelStatus {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_GetPendingDirectDebits",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingGetPendingDirectDebits {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: DirectDebitPendingPaymentRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DirectDebitPendingPaymentRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct DirectDebitPendingPaymentRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<i32>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<i32>,
        #[yaserde(rename = "IncludeSubAccounts", prefix = "tns", default)]
        pub include_sub_accounts: bool,
        #[yaserde(rename = "IncludeAll", prefix = "tns", default)]
        pub include_all: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_GetPendingDirectDebitsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingGetPendingDirectDebitsResponse {
        #[yaserde(
            rename = "Ws_Banking_GetPendingDirectDebitsResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_get_pending_direct_debits_result: DirectDebitPendingPaymentResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DirectDebitPendingPaymentResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct DirectDebitPendingPaymentResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "PendingDirectDebits", prefix = "tns", default)]
        pub pending_direct_debits: Option<ArrayOfPendingDirectDebit>,
        #[yaserde(rename = "TotalValueOfTransactions", prefix = "tns", default)]
        pub total_value_of_transactions: f64,
        #[yaserde(rename = "TotalValueOfFees", prefix = "tns", default)]
        pub total_value_of_fees: f64,
        #[yaserde(rename = "TotalValueOfTransactionsAndFees", prefix = "tns", default)]
        pub total_value_of_transactions_and_fees: f64,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPendingDirectDebit",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfPendingDirectDebit {
        #[yaserde(rename = "PendingDirectDebit", prefix = "tns", default)]
        pub pending_direct_debit: Vec<PendingDirectDebit>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PendingDirectDebit",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PendingDirectDebit {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "BLGUID", prefix = "tns", default)]
        pub blguid: Option<String>,
        #[yaserde(rename = "IBAN", prefix = "tns", default)]
        pub iban: Option<String>,
        #[yaserde(rename = "Amount", prefix = "tns", default)]
        pub amount: f64,
        #[yaserde(rename = "FeeAmount", prefix = "tns", default)]
        pub fee_amount: f64,
        #[yaserde(rename = "TotalAmount", prefix = "tns", default)]
        pub total_amount: f64,
        #[yaserde(rename = "CreditorReference", prefix = "tns", default)]
        pub creditor_reference: Option<String>,
        #[yaserde(rename = "Reference", prefix = "tns", default)]
        pub reference: Option<String>,
        #[yaserde(rename = "AvailableAmount", prefix = "tns", default)]
        pub available_amount: f64,
        #[yaserde(rename = "PendingSuccess", prefix = "tns", default)]
        pub pending_success: EnumPendingDirectDebit,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_PendingDirectDebit",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumPendingDirectDebit {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_Card_Statement_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCardStatementV2 {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: WsBankingCardStatementV2Request,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WsBankingCardStatementV2Request",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCardStatementV2Request {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "TxnFilter", prefix = "tns", default)]
        pub txn_filter: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", prefix = "tns", default)]
        pub num_txn: i32,
        #[yaserde(rename = "DataSrc", prefix = "tns", default)]
        pub data_src: i32,
        #[yaserde(rename = "DescriptionDelimiter", prefix = "tns", default)]
        pub description_delimiter: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_Card_Statement_V2Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingCardStatementV2Response {
        #[yaserde(rename = "Ws_Banking_Card_Statement_V2Result", prefix = "tns", default)]
        pub ws_banking_card_statement_v2_result: BankingCardStatementV2Response,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BankingCardStatementV2Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BankingCardStatementV2Response {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "Statements", prefix = "tns", default)]
        pub statements: Option<ArrayOfCardStatement2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardStatement2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardStatement2 {
        #[yaserde(rename = "CardStatement2", prefix = "tns", default)]
        pub card_statement_2: Vec<CardStatement2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardStatement2",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct CardStatement2 {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "StartBal", default)]
        pub start_bal: String,
        #[yaserde(rename = "EndBal", default)]
        pub end_bal: String,
        #[yaserde(rename = "TxnFilter", default)]
        pub txn_filter: Option<String>,
        #[yaserde(rename = "StartDate", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", default)]
        pub num_txn: i32,
        #[yaserde(rename = "ItemSrc", default)]
        pub item_src: i32,
        #[yaserde(rename = "CurBill", default)]
        pub cur_bill: Option<String>,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "BlkAmt", default)]
        pub blk_amt: String,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Transactions", default)]
        pub transactions: Option<ArrayOfTransaction2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ArrayOfTransaction2")]
    pub struct ArrayOfTransaction2 {
        #[yaserde(rename = "Transaction2", default)]
        pub transaction_2: Vec<Transaction2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "Transaction2")]
    pub struct Transaction2 {
        #[yaserde(rename = "TxnDate", default)]
        pub txn_date: Option<String>,
        #[yaserde(rename = "PostDate", default)]
        pub post_date: Option<String>,
        #[yaserde(rename = "AmtBill", default)]
        pub amt_bill: String,
        #[yaserde(rename = "AmtTxn", default)]
        pub amt_txn: String,
        #[yaserde(rename = "BillConvRate", default)]
        pub bill_conv_rate: String,
        #[yaserde(rename = "DebOrCred", default)]
        pub deb_or_cred: i32,
        #[yaserde(rename = "TerminalId", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "Description", default)]
        pub description: Option<String>,
        #[yaserde(rename = "RRN", default)]
        pub rrn: Option<String>,
        #[yaserde(rename = "CurTxn", default)]
        pub cur_txn: Option<String>,
        #[yaserde(rename = "ItemId", default)]
        pub item_id: i64,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "BlkAmt", default)]
        pub blk_amt: String,
        #[yaserde(rename = "TransactionType", default)]
        pub transaction_type: Option<String>,
        #[yaserde(rename = "StatusCode", default)]
        pub status_code: Option<String>,
        #[yaserde(rename = "StatusDesc", default)]
        pub status_desc: Option<String>,
        #[yaserde(rename = "TxnTime", default)]
        pub txn_time: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "FeeId", default)]
        pub fee_id: i64,
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "FixedFee", default)]
        pub fixed_fee: String,
        #[yaserde(rename = "RateFee", default)]
        pub rate_fee: String,
        #[yaserde(rename = "FxPdg", default)]
        pub fx_pdg: String,
        #[yaserde(rename = "MCCPdg", default)]
        pub mcc_pdg: String,
        #[yaserde(rename = "LinkId", default)]
        pub link_id: Option<String>,
        #[yaserde(rename = "MCC", default)]
        pub mcc: Option<String>,
        #[yaserde(rename = "OrigStan", default)]
        pub orig_stan: Option<String>,
        #[yaserde(rename = "ProcCode", default)]
        pub proc_code: Option<String>,
        #[yaserde(rename = "MCCDescription", default)]
        pub mcc_description: Option<String>,
        #[yaserde(rename = "Dom_Fee_Fixed", default)]
        pub dom_fee_fixed: Option<String>,
        #[yaserde(rename = "Dom_Fee_Rate", default)]
        pub dom_fee_rate: Option<String>,
        #[yaserde(rename = "Non_Dom_Fee_Fixed", default)]
        pub non_dom_fee_fixed: Option<String>,
        #[yaserde(rename = "Non_Dom_Fee_Rate", default)]
        pub non_dom_fee_rate: Option<String>,
        #[yaserde(rename = "Fx_Fee_Fixed", default)]
        pub fx_fee_fixed: Option<String>,
        #[yaserde(rename = "Fx_Fee_Rate", default)]
        pub fx_fee_rate: Option<String>,
        #[yaserde(rename = "Other_Fee_Desc", default)]
        pub other_fee_desc: Option<String>,
        #[yaserde(rename = "Other_Fee_Amt", default)]
        pub other_fee_amt: Option<String>,
        #[yaserde(rename = "TxnCodeType", default)]
        pub txn_code_type: Option<String>,
        #[yaserde(rename = "Note", default)]
        pub note: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Insert3DSecureDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsInsert3DSecureDetails {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Option<String>,
        #[yaserde(rename = "MemorableName", prefix = "tns", default)]
        pub memorable_name: Option<String>,
        #[yaserde(rename = "MemorablePlace", prefix = "tns", default)]
        pub memorable_place: Option<String>,
        #[yaserde(rename = "MemorableDate", prefix = "tns", default)]
        pub memorable_date: Option<String>,
        #[yaserde(rename = "ActivationCode", prefix = "tns", default)]
        pub activation_code: Option<String>,
        #[yaserde(rename = "Phone", prefix = "tns", default)]
        pub phone: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Insert3DSecureDetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsInsert3DSecureDetailsResponse {
        #[yaserde(rename = "Ws_Insert3DSecureDetailsResult", prefix = "tns", default)]
        pub ws_insert_3d_secure_details_result: Gps3Dsecure,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GPS3DSecure",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Gps3Dsecure {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Update3DSecureDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdate3DSecureDetails {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Option<String>,
        #[yaserde(rename = "MemorableName", prefix = "tns", default)]
        pub memorable_name: Option<String>,
        #[yaserde(rename = "MemorablePlace", prefix = "tns", default)]
        pub memorable_place: Option<String>,
        #[yaserde(rename = "MemorableDate", prefix = "tns", default)]
        pub memorable_date: Option<String>,
        #[yaserde(rename = "ActivationCode", prefix = "tns", default)]
        pub activation_code: Option<String>,
        #[yaserde(rename = "Phone", prefix = "tns", default)]
        pub phone: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Update3DSecureDetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdate3DSecureDetailsResponse {
        #[yaserde(rename = "Ws_Update3DSecureDetailsResult", prefix = "tns", default)]
        pub ws_update_3d_secure_details_result: Gps3Dsecure,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UpdateLastModifiedType",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateLastModifiedType {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "LastModifiedType", prefix = "tns", default)]
        pub last_modified_type: Option<String>,
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UpdateLastModifiedTypeResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateLastModifiedTypeResponse {
        #[yaserde(rename = "Ws_UpdateLastModifiedTypeResult", prefix = "tns", default)]
        pub ws_update_last_modified_type_result: Gps3Dsecure,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Delete3DSecureDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsDelete3DSecureDetails {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Delete3DSecureDetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsDelete3DSecureDetailsResponse {
        #[yaserde(rename = "Ws_Delete3DSecureDetailsResult", prefix = "tns", default)]
        pub ws_delete_3d_secure_details_result: Gps3Dsecure,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Query3DSecureDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsQuery3DSecureDetails {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "QueryType", prefix = "tns", default)]
        pub query_type: Option<String>,
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Query3DSecureDetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsQuery3DSecureDetailsResponse {
        #[yaserde(rename = "Ws_Query3DSecureDetailsResult", prefix = "tns", default)]
        pub ws_query_3d_secure_details_result: Query3DSecure,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Query3DSecure",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Query3DSecure {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Tokens", prefix = "tns", default)]
        pub tokens: Option<ArrayOfToken>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfToken",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfToken {
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Vec<Token>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Token",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Token {
        #[yaserde(rename = "Token", prefix = "tns", default)]
        pub token: Option<String>,
        #[yaserde(rename = "Phone", prefix = "tns", default)]
        pub phone: Option<String>,
        #[yaserde(rename = "ResponseCode", prefix = "tns", default)]
        pub response_code: Option<String>,
        #[yaserde(rename = "LastModifiedType", prefix = "tns", default)]
        pub last_modified_type: Option<String>,
        #[yaserde(rename = "ResponseDescription", prefix = "tns", default)]
        pub response_description: Option<String>,
        #[yaserde(rename = "LastApprovedPhone", prefix = "tns", default)]
        pub last_approved_phone: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GPS_Lock_Unlock",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGPSLockUnlock {
        #[yaserde(rename = "LockMode", prefix = "tns", default)]
        pub lock_mode: Option<String>,
        #[yaserde(rename = "Process_Name", prefix = "tns", default)]
        pub process_name: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GPS_Lock_UnlockResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGPSLockUnlockResponse {
        #[yaserde(rename = "Ws_GPS_Lock_UnlockResult", prefix = "tns", default)]
        pub ws_gps_lock_unlock_result: GpsLockUnlock,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GPS_Lock_Unlock",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GpsLockUnlock {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LockStatus", prefix = "tns", default)]
        pub lock_status: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "W2DetailsPassThroughDataRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct W2DetailsPassThroughDataRequest {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WS_VerificationRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsVerificationRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "UserID", prefix = "tns", default)]
        pub user_id: Option<String>,
        #[yaserde(rename = "W2DetailsPassThroughDataRequest", prefix = "tns", default)]
        pub w2_details_pass_through_data_request: Option<W2DetailsPassThroughDataRequest>,
        #[yaserde(rename = "GPSSanctionsPEPCheckDetails", prefix = "tns", default)]
        pub gps_sanctions_pep_check_details: GpssanctionsPEP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GPSSanctionsPEP",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GpssanctionsPEP {
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Nationality", prefix = "tns", default)]
        pub nationality: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "CheckLevel", prefix = "tns", default)]
        pub check_level: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WS_VerificationRequestResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsVerificationRequestResponse {
        #[yaserde(rename = "WS_VerificationRequestResult", prefix = "tns", default)]
        pub ws_verification_request_result: GpsKycVerificationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GPS_KYC_Verification_Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GpsKycVerificationResponse {
        #[yaserde(rename = "WSSID", prefix = "tns", default)]
        pub wssid: Option<String>,
        #[yaserde(rename = "ISSCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "GPSSanctionsPEPResponse", prefix = "tns", default)]
        pub gps_sanctions_pep_response: SanctionsPEPV2,
        #[yaserde(rename = "W2Response", prefix = "tns", default)]
        pub w2_response: W2PassThroughDetailsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Sanctions_PEP_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct SanctionsPEPV2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: i32,
        #[yaserde(rename = "PEPID", prefix = "tns", default)]
        pub pepid: Option<String>,
        #[yaserde(rename = "CheckLevel", prefix = "tns", default)]
        pub check_level: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "CompleteMatch", prefix = "tns", default)]
        pub complete_match: bool,
        #[yaserde(rename = "Source", prefix = "tns", default)]
        pub source: Option<String>,
        #[yaserde(rename = "MatchItems", prefix = "tns", default)]
        pub match_items: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "W2DetailResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct W2DetailResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "W2PassThroughDetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct W2PassThroughDetailsResponse {
        #[yaserde(rename = "WorkflowName", prefix = "tns", default)]
        pub workflow_name: Option<String>,
        #[yaserde(rename = "IDValue", prefix = "tns", default)]
        pub id_value: Option<String>,
        #[yaserde(rename = "W2DetailResponse", prefix = "tns", default)]
        pub w2_detail_response: Option<W2DetailResponse>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_AddUpDelCredentials",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsAddUpDelCredentials {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Action", prefix = "tns", default)]
        pub action: Actions,
        #[yaserde(rename = "Credentials", prefix = "tns", default)]
        pub credentials: Option<ArrayOfCredential>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Actions",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Actions {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCredential",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCredential {
        #[yaserde(rename = "Credential", prefix = "tns", default)]
        pub credential: Vec<Credential>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Credential",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Credential {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: i32,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: CredentialTypes,
        #[yaserde(rename = "Value", prefix = "tns", default)]
        pub value: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CredentialTypes",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CredentialTypes {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_AddUpDelCredentialsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsAddUpDelCredentialsResponse {
        #[yaserde(rename = "Ws_AddUpDelCredentialsResult", prefix = "tns", default)]
        pub ws_add_up_del_credentials_result: AddUpDelCredentialsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddUpDelCredentialsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AddUpDelCredentialsResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Action", prefix = "tns", default)]
        pub action: Actions,
        #[yaserde(rename = "Credentials", prefix = "tns", default)]
        pub credentials: Option<ArrayOfCredential>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_3DS_AddUpDelDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Ws3DSAddUpDelDetails {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "RegisterDetails", prefix = "tns", default)]
        pub register_details: bool,
        #[yaserde(rename = "DetailsStatus", prefix = "tns", default)]
        pub details_status: Enum3DsAction,
        #[yaserde(rename = "Details", prefix = "tns", default)]
        pub details: Option<ArrayOfDetail>,
        #[yaserde(rename = "RegisterSMS", prefix = "tns", default)]
        pub register_sms: bool,
        #[yaserde(rename = "SMSStatus", prefix = "tns", default)]
        pub sms_status: Enum3DsAction,
        #[yaserde(rename = "MobileNumber", prefix = "tns", default)]
        pub mobile_number: Option<String>,
        #[yaserde(rename = "Deregister", prefix = "tns", default)]
        pub deregister: bool,
        #[yaserde(rename = "OverrideGPS", prefix = "tns", default)]
        pub override_gps: bool,
        #[yaserde(rename = "ReIssueBoolean", prefix = "tns", default)]
        pub re_issue_boolean: bool,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_3DS_Action",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Enum3DsAction {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfDetail",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfDetail {
        #[yaserde(rename = "Detail", prefix = "tns", default)]
        pub detail: Vec<Detail>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Detail",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Detail {
        #[yaserde(rename = "Identity", prefix = "tns", default)]
        pub identity: Enum3DsDetails,
        #[yaserde(rename = "Value", prefix = "tns", default)]
        pub value: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_3DS_Details",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Enum3DsDetails {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_3DS_AddUpDelDetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Ws3DSAddUpDelDetailsResponse {
        #[yaserde(rename = "Ws_3DS_AddUpDelDetailsResult", prefix = "tns", default)]
        pub ws_3ds_add_up_del_details_result: Ws3DSAddUpDelDetailsResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_3DS_AddUpDelDetailsResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Ws3DSAddUpDelDetailsResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BalanceUpdate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceUpdate {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "AvlBalance_GPS_STIP", prefix = "tns", default)]
        pub avl_balance_gps_stip: f64,
        #[yaserde(rename = "CurBalance_GPS_STIP", prefix = "tns", default)]
        pub cur_balance_gps_stip: f64,
        #[yaserde(rename = "Balance_Sequence_Exthost", prefix = "tns", default)]
        pub balance_sequence_exthost: i64,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BalanceUpdateResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceUpdateResponse {
        #[yaserde(rename = "Ws_BalanceUpdateResult", prefix = "tns", default)]
        pub ws_balance_update_result: BalanceUpdateResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BalanceUpdateResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BalanceUpdateResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Balance_Sequence_Exthost", prefix = "tns", default)]
        pub balance_sequence_exthost: i64,
        #[yaserde(rename = "Balance_Sequence", prefix = "tns", default)]
        pub balance_sequence: i64,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Payment_Token_Get",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPaymentTokenGet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DPAN", prefix = "tns", default)]
        pub dpan: Option<String>,
        #[yaserde(rename = "Payment_Token_ID", prefix = "tns", default)]
        pub payment_token_id: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Payment_Token_GetResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPaymentTokenGetResponse {
        #[yaserde(rename = "Ws_Payment_Token_GetResult", prefix = "tns", default)]
        pub ws_payment_token_get_result: PaymentTokenGetRes,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PaymentTokenGetRes",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PaymentTokenGetRes {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "PaymentTokenGetResInfo", prefix = "tns", default)]
        pub payment_token_get_res_info: Option<ArrayOfPaymentTokenGetResInfo>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPaymentTokenGetResInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfPaymentTokenGetResInfo {
        #[yaserde(rename = "PaymentTokenGetResInfo", prefix = "tns", default)]
        pub payment_token_get_res_info: Vec<PaymentTokenGetResInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PaymentTokenGetResInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PaymentTokenGetResInfo {
        #[yaserde(rename = "Creator", prefix = "tns", default)]
        pub creator: Option<String>,
        #[yaserde(rename = "Creator_PAN_Ref", prefix = "tns", default)]
        pub creator_pan_ref: Option<String>,
        #[yaserde(rename = "Creator_Token_Ref", prefix = "tns", default)]
        pub creator_token_ref: Option<String>,
        #[yaserde(rename = "PANT", prefix = "tns", default)]
        pub pant: Option<String>,
        #[yaserde(rename = "Payment_Token", prefix = "tns", default)]
        pub payment_token: Option<String>,
        #[yaserde(rename = "Payment_Token_ExpDate", prefix = "tns", default)]
        pub payment_token_exp_date: Option<String>,
        #[yaserde(rename = "Payment_Token_ID", prefix = "tns", default)]
        pub payment_token_id: Option<String>,
        #[yaserde(rename = "Payment_Token_Type", prefix = "tns", default)]
        pub payment_token_type: Option<String>,
        #[yaserde(rename = "Wallet_ID", prefix = "tns", default)]
        pub wallet_id: Option<String>,
        #[yaserde(rename = "GPS_Status", prefix = "tns", default)]
        pub gps_status: Option<String>,
        #[yaserde(rename = "Tokenised_Datetime", prefix = "tns", default)]
        pub tokenised_datetime: Option<String>,
        #[yaserde(rename = "Tokenised_Status", prefix = "tns", default)]
        pub tokenised_status: Option<String>,
        #[yaserde(rename = "Txn_Status", prefix = "tns", default)]
        pub txn_status: Option<String>,
        #[yaserde(rename = "Txn_Status_Actor", prefix = "tns", default)]
        pub txn_status_actor: Option<String>,
        #[yaserde(rename = "Txn_Status_Change_Datetime", prefix = "tns", default)]
        pub txn_status_change_datetime: Option<String>,
        #[yaserde(rename = "Accepted_Terms_Date_GMT", prefix = "tns", default)]
        pub accepted_terms_date_gmt: Option<String>,
        #[yaserde(rename = "Accepted_Terms_Version", prefix = "tns", default)]
        pub accepted_terms_version: Option<String>,
        #[yaserde(rename = "Auth_Datetime", prefix = "tns", default)]
        pub auth_datetime: Option<String>,
        #[yaserde(rename = "Auth_Decision", prefix = "tns", default)]
        pub auth_decision: Option<String>,
        #[yaserde(rename = "Auth_RSPSRC", prefix = "tns", default)]
        pub auth_rspsrc: Option<String>,
        #[yaserde(rename = "Auth_Status", prefix = "tns", default)]
        pub auth_status: Option<String>,
        #[yaserde(rename = "Digitisation_Ref", prefix = "tns", default)]
        pub digitisation_ref: Option<String>,
        #[yaserde(rename = "Wallet_Account_Score", prefix = "tns", default)]
        pub wallet_account_score: Option<String>,
        #[yaserde(rename = "Wallet_Device_Score", prefix = "tns", default)]
        pub wallet_device_score: Option<String>,
        #[yaserde(rename = "Wallet_Reasons", prefix = "tns", default)]
        pub wallet_reasons: Option<String>,
        #[yaserde(rename = "Activation_Code", prefix = "tns", default)]
        pub activation_code: Option<String>,
        #[yaserde(rename = "Activation_Code_Expdate", prefix = "tns", default)]
        pub activation_code_expdate: Option<String>,
        #[yaserde(rename = "Activation_Method", prefix = "tns", default)]
        pub activation_method: Option<String>,
        #[yaserde(rename = "Device_ID", prefix = "tns", default)]
        pub device_id: Option<String>,
        #[yaserde(rename = "Device_IP", prefix = "tns", default)]
        pub device_ip: Option<String>,
        #[yaserde(rename = "Device_Lang2", prefix = "tns", default)]
        pub device_lang_2: Option<String>,
        #[yaserde(rename = "Device_Latitude", prefix = "tns", default)]
        pub device_latitude: Option<String>,
        #[yaserde(rename = "Device_Longitude", prefix = "tns", default)]
        pub device_longitude: Option<String>,
        #[yaserde(rename = "Device_Name", prefix = "tns", default)]
        pub device_name: Option<String>,
        #[yaserde(rename = "Device_Tel_Num", prefix = "tns", default)]
        pub device_tel_num: Option<String>,
        #[yaserde(rename = "Device_Type", prefix = "tns", default)]
        pub device_type: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Wallet_Account_Hash", prefix = "tns", default)]
        pub wallet_account_hash: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Payment_Token_StatusChange",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPaymentTokenStatusChange {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DPAN", prefix = "tns", default)]
        pub dpan: Option<String>,
        #[yaserde(rename = "PaymentTokenId", prefix = "tns", default)]
        pub payment_token_id: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "NewStatCode", prefix = "tns", default)]
        pub new_stat_code: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Payment_Token_StatusChangeResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPaymentTokenStatusChangeResponse {
        #[yaserde(
            rename = "Ws_Payment_Token_StatusChangeResult",
            prefix = "tns",
            default
        )]
        pub ws_payment_token_status_change_result: PaymentTokenStatusChangeResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PaymentTokenStatusChangeResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PaymentTokenStatusChangeResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "PaymentTokenId", prefix = "tns", default)]
        pub payment_token_id: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "NetworkError", prefix = "tns", default)]
        pub network_error: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Activate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsActivate {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "ActMethod", prefix = "tns", default)]
        pub act_method: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "SMSBalance", prefix = "tns", default)]
        pub sms_balance: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_ActivateResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsActivateResponse {
        #[yaserde(rename = "Ws_ActivateResult", default)]
        pub ws_activate_result: Activate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Activate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Activate {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "IsLive", default)]
        pub is_live: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Load",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: Option<String>,
        #[yaserde(rename = "BrnCode", prefix = "tns", default)]
        pub brn_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_LoadResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsLoadResponse {
        #[yaserde(rename = "Ws_LoadResult", default)]
        pub ws_load_result: LoadCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LoadCard")]
    pub struct LoadCard {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemID", default)]
        pub item_id: i64,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UnLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUnLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "AmtUnLoad", prefix = "tns", default)]
        pub amt_un_load: f64,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UnLoadResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsUnLoadResponse {
        #[yaserde(rename = "Ws_UnLoadResult", default)]
        pub ws_un_load_result: UnLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UnLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct UnLoad {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AmtUnLoad", default)]
        pub amt_un_load: f64,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "BlkAmt", default)]
        pub blk_amt: String,
        #[yaserde(rename = "ItemID", default)]
        pub item_id: i64,
        #[yaserde(rename = "CurCode", default)]
        pub cur_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_StatusChange",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsStatusChange {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "NewStatCode", prefix = "tns", default)]
        pub new_stat_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_StatusChangeResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsStatusChangeResponse {
        #[yaserde(rename = "Ws_StatusChangeResult", default)]
        pub ws_status_change_result: StatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "StatusChange")]
    pub struct StatusChange {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "AccNo", prefix = "tns", default)]
        pub acc_no: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "SecId", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_EnquiryResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsEnquiryResponse {
        #[yaserde(rename = "Ws_EnquiryResult", default)]
        pub ws_enquiry_result: Card2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Card2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Card2 {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "StartDate", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "ExpDate", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "StatCode", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "EmbossName", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "BlkAmt", default)]
        pub blk_amt: String,
        #[yaserde(rename = "CurCode", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "IsLive", default)]
        pub is_live: bool,
        #[yaserde(rename = "Scheme", default)]
        pub scheme: Option<String>,
        #[yaserde(rename = "Product", default)]
        pub product: Option<String>,
        #[yaserde(rename = "MaskedPAN", default)]
        pub masked_pan: Option<String>,
        #[yaserde(rename = "LimitGroup", default)]
        pub limit_group: Option<String>,
        #[yaserde(rename = "MCCGroup", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "FeeGroup", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "LinkageGroup", default)]
        pub linkage_group: Option<String>,
        #[yaserde(rename = "PrimaryToken", default)]
        pub primary_token: Option<String>,
        #[yaserde(rename = "AuthCalendarGroup", default)]
        pub auth_calendar_group: Option<String>,
        #[yaserde(rename = "FXGroup", default)]
        pub fx_group: Option<String>,
        #[yaserde(rename = "BlackList", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "WhiteList", default)]
        pub white_list: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", default)]
        pub payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BalanceTransfer",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceTransfer {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "NewPAN", prefix = "tns", default)]
        pub new_pan: Option<String>,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: Option<String>,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
        #[yaserde(rename = "BrnCode", prefix = "tns", default)]
        pub brn_code: Option<String>,
        #[yaserde(rename = "Fee", prefix = "tns", default)]
        pub fee: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BalanceTransferResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceTransferResponse {
        #[yaserde(rename = "Ws_BalanceTransferResult", prefix = "tns", default)]
        pub ws_balance_transfer_result: BalanceTransfer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BalanceTransfer",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BalanceTransfer {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "NewPAN", prefix = "tns", default)]
        pub new_pan: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "GetLimits", prefix = "tns", default)]
        pub get_limits: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_EnquiryResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsBalanceEnquiryResponse {
        #[yaserde(rename = "Ws_Balance_EnquiryResult", default)]
        pub ws_balance_enquiry_result: BalanceEnquire,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "BalanceEnquire")]
    pub struct BalanceEnquire {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "BlkAmt", default)]
        pub blk_amt: String,
        #[yaserde(rename = "CurCode", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PINStatus", default)]
        pub pin_status: i32,
        #[yaserde(rename = "LimitInfo", default)]
        pub limit_info: Option<ArrayOfLimitInformation>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfLimitInformation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfLimitInformation {
        #[yaserde(rename = "LimitInformation", prefix = "tns", default)]
        pub limit_information: Vec<LimitInformation>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LimitInformation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct LimitInformation {
        #[yaserde(rename = "MaxAllowableBalance", prefix = "tns", default)]
        pub max_allowable_balance: f64,
        #[yaserde(rename = "DailyLoadLimit", prefix = "tns", default)]
        pub daily_load_limit: f64,
        #[yaserde(rename = "DailyLoadFrequencyLimit", prefix = "tns", default)]
        pub daily_load_frequency_limit: i32,
        #[yaserde(rename = "AmountLoaded", prefix = "tns", default)]
        pub amount_loaded: f64,
        #[yaserde(rename = "NoOfLoads", prefix = "tns", default)]
        pub no_of_loads: i32,
        #[yaserde(rename = "AmountLeftToLoad", prefix = "tns", default)]
        pub amount_left_to_load: f64,
        #[yaserde(rename = "NoOfLoadLeft", prefix = "tns", default)]
        pub no_of_load_left: i32,
        #[yaserde(rename = "DailyPosLimit", prefix = "tns", default)]
        pub daily_pos_limit: f64,
        #[yaserde(rename = "DailyPosFrequencyLimit", prefix = "tns", default)]
        pub daily_pos_frequency_limit: i32,
        #[yaserde(rename = "POSUsage", prefix = "tns", default)]
        pub pos_usage: f64,
        #[yaserde(rename = "NoOfPOSUsage", prefix = "tns", default)]
        pub no_of_pos_usage: i32,
        #[yaserde(rename = "ValueOfPOSLeft", prefix = "tns", default)]
        pub value_of_pos_left: f64,
        #[yaserde(rename = "NoOfPOSTransactionsLeft", prefix = "tns", default)]
        pub no_of_pos_transactions_left: i32,
        #[yaserde(rename = "DailyCashLimit", prefix = "tns", default)]
        pub daily_cash_limit: f64,
        #[yaserde(rename = "DailyCashFrequencyLimit", prefix = "tns", default)]
        pub daily_cash_frequency_limit: i32,
        #[yaserde(rename = "CashWithdrawal", prefix = "tns", default)]
        pub cash_withdrawal: f64,
        #[yaserde(rename = "NoOfCashWithdrawal", prefix = "tns", default)]
        pub no_of_cash_withdrawal: i32,
        #[yaserde(rename = "ValueOfCashLeft", prefix = "tns", default)]
        pub value_of_cash_left: f64,
        #[yaserde(rename = "NoOfCashTransactionLeft", prefix = "tns", default)]
        pub no_of_cash_transaction_left: i32,
        #[yaserde(rename = "DailyUnLoadLimit", prefix = "tns", default)]
        pub daily_un_load_limit: f64,
        #[yaserde(rename = "DailyUnLoadFrequencyLimit", prefix = "tns", default)]
        pub daily_un_load_frequency_limit: i32,
        #[yaserde(rename = "AmountUnLoaded", prefix = "tns", default)]
        pub amount_un_loaded: f64,
        #[yaserde(rename = "NoOfUnLoads", prefix = "tns", default)]
        pub no_of_un_loads: i32,
        #[yaserde(rename = "AmountLeftToUnLoad", prefix = "tns", default)]
        pub amount_left_to_un_load: f64,
        #[yaserde(rename = "NoOfUnLoadLeft", prefix = "tns", default)]
        pub no_of_un_load_left: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_Rep",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceEnquiryRep {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "GetLimits", prefix = "tns", default)]
        pub get_limits: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_RepResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceEnquiryRepResponse {
        #[yaserde(rename = "Ws_Balance_Enquiry_RepResult", prefix = "tns", default)]
        pub ws_balance_enquiry_rep_result: BalanceEnquire,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns",
        default_namespace = "tns"
    )]
    pub struct WsBalanceEnquiryV2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "GetLimits", prefix = "tns", default)]
        pub get_limits: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_V2Response",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsBalanceEnquiryV2Response {
        #[yaserde(rename = "Ws_Balance_Enquiry_V2Result", default)]
        pub ws_balance_enquiry_v2_result: Option<BalanceEnquire2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LimitInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct LimitInfo {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BalanceEnquire2",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct BalanceEnquire2 {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "BlkAmt", default)]
        pub blk_amt: String,
        #[yaserde(rename = "CurCode", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PINStatus", default)]
        pub pin_status: i32,
        #[yaserde(rename = "LimitInfo", default)]
        pub limit_info: Option<LimitInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_Wallet",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceEnquiryWallet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Balance_Enquiry_WalletResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceEnquiryWalletResponse {
        #[yaserde(rename = "Ws_Balance_Enquiry_WalletResult", prefix = "tns", default)]
        pub ws_balance_enquiry_wallet_result: BalanceEnquireWallet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "EWALLET",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Ewallet {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BalanceEnquireWallet",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BalanceEnquireWallet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PINStatus", prefix = "tns", default)]
        pub pin_status: i32,
        #[yaserde(rename = "EWALLET", prefix = "tns", default)]
        pub ewallet: Option<Ewallet>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Statement",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardStatement {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "TxnFilter", prefix = "tns", default)]
        pub txn_filter: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", prefix = "tns", default)]
        pub num_txn: i32,
        #[yaserde(rename = "DataSrc", prefix = "tns", default)]
        pub data_src: i32,
        #[yaserde(rename = "DescriptionDelimiter", prefix = "tns", default)]
        pub description_delimiter: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_StatementResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsCardStatementResponse {
        #[yaserde(rename = "Ws_Card_StatementResult", default)]
        pub ws_card_statement_result: CardStatement2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Statement_Rep",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardStatementRep {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "TxnFilter", prefix = "tns", default)]
        pub txn_filter: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", prefix = "tns", default)]
        pub num_txn: i32,
        #[yaserde(rename = "DataSrc", prefix = "tns", default)]
        pub data_src: i32,
        #[yaserde(rename = "DescriptionDelimiter", prefix = "tns", default)]
        pub description_delimiter: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Statement_RepResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardStatementRepResponse {
        #[yaserde(rename = "Ws_Card_Statement_RepResult", prefix = "tns", default)]
        pub ws_card_statement_rep_result: CardStatement2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Customer_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCustomerEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "AccNo", prefix = "tns", default)]
        pub acc_no: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Customer_EnquiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCustomerEnquiryResponse {
        #[yaserde(rename = "Ws_Customer_EnquiryResult", prefix = "tns", default)]
        pub ws_customer_enquiry_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Customer",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Customer {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "AccNo", prefix = "tns", default)]
        pub acc_no: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Cards", prefix = "tns", default)]
        pub cards: Option<ArrayOfCardList>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardList {
        #[yaserde(rename = "CardList", prefix = "tns", default)]
        pub card_list: Vec<CardList>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardList {
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "CrdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "Program", prefix = "tns", default)]
        pub program: Option<String>,
        #[yaserde(rename = "DesignRef", prefix = "tns", default)]
        pub design_ref: Option<String>,
        #[yaserde(rename = "EmbossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "CustType", prefix = "tns", default)]
        pub cust_type: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Customer_Enquiry_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCustomerEnquiryV2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Customer_Enquiry_V2Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCustomerEnquiryV2Response {
        #[yaserde(rename = "Ws_Customer_Enquiry_V2Result", prefix = "tns", default)]
        pub ws_customer_enquiry_v2_result: Customer2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Customer2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Customer2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Primary", prefix = "tns", default)]
        pub primary: Option<String>,
        #[yaserde(rename = "Cards", prefix = "tns", default)]
        pub cards: Option<ArrayOfCardList2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardList2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardList2 {
        #[yaserde(rename = "CardList2", prefix = "tns", default)]
        pub card_list_2: Vec<CardList2>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardList2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardList2 {
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "CrdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "Program", prefix = "tns", default)]
        pub program: Option<String>,
        #[yaserde(rename = "DesignRef", prefix = "tns", default)]
        pub design_ref: Option<String>,
        #[yaserde(rename = "EmbossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "CustType", prefix = "tns", default)]
        pub cust_type: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Primary", prefix = "tns", default)]
        pub primary: Option<String>,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: i32,
        #[yaserde(rename = "Scheme", prefix = "tns", default)]
        pub scheme: Option<String>,
        #[yaserde(rename = "Product", prefix = "tns", default)]
        pub product: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "LimitGroup", prefix = "tns", default)]
        pub limit_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", prefix = "tns", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WsFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "LinkageGroup", prefix = "tns", default)]
        pub linkage_group: Option<String>,
        #[yaserde(rename = "PrimaryToken", prefix = "tns", default)]
        pub primary_token: Option<String>,
        #[yaserde(rename = "MaskedPAN", prefix = "tns", default)]
        pub masked_pan: Option<String>,
        #[yaserde(rename = "ProductID", prefix = "tns", default)]
        pub product_id: Option<String>,
        #[yaserde(rename = "ProductRef", prefix = "tns", default)]
        pub product_ref: Option<String>,
        #[yaserde(rename = "AuthCalendarGroup", prefix = "tns", default)]
        pub auth_calendar_group: Option<String>,
        #[yaserde(rename = "FXGroup", prefix = "tns", default)]
        pub fx_group: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", prefix = "tns", default)]
        pub payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Update_Cardholder_Details",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateCardholderDetails {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "accCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "newAccCode", prefix = "tns", default)]
        pub new_acc_code: Option<String>,
        #[yaserde(rename = "crdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "lastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "firstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "addr1", prefix = "tns", default)]
        pub addr_1: Option<String>,
        #[yaserde(rename = "addr2", prefix = "tns", default)]
        pub addr_2: Option<String>,
        #[yaserde(rename = "city", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "postcode", prefix = "tns", default)]
        pub postcode: Option<String>,
        #[yaserde(rename = "country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "tel", prefix = "tns", default)]
        pub tel: Option<String>,
        #[yaserde(rename = "Workaddr1", prefix = "tns", default)]
        pub workaddr_1: Option<String>,
        #[yaserde(rename = "Workaddr2", prefix = "tns", default)]
        pub workaddr_2: Option<String>,
        #[yaserde(rename = "Workaddr3", prefix = "tns", default)]
        pub workaddr_3: Option<String>,
        #[yaserde(rename = "Workcity", prefix = "tns", default)]
        pub workcity: Option<String>,
        #[yaserde(rename = "Workpostcode", prefix = "tns", default)]
        pub workpostcode: Option<String>,
        #[yaserde(rename = "Workcounty", prefix = "tns", default)]
        pub workcounty: Option<String>,
        #[yaserde(rename = "Workcountry", prefix = "tns", default)]
        pub workcountry: Option<String>,
        #[yaserde(rename = "Worktel", prefix = "tns", default)]
        pub worktel: Option<String>,
        #[yaserde(rename = "pobox", prefix = "tns", default)]
        pub pobox: Option<String>,
        #[yaserde(rename = "email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "fax", prefix = "tns", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "mobTel", prefix = "tns", default)]
        pub mob_tel: Option<String>,
        #[yaserde(rename = "maritalStatus", prefix = "tns", default)]
        pub marital_status: Option<String>,
        #[yaserde(rename = "sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "embossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "refuseCheck", prefix = "tns", default)]
        pub refuse_check: Option<String>,
        #[yaserde(rename = "mailShots", prefix = "tns", default)]
        pub mail_shots: Option<String>,
        #[yaserde(rename = "discret", prefix = "tns", default)]
        pub discret: Option<String>,
        #[yaserde(rename = "userdata", prefix = "tns", default)]
        pub userdata: Option<String>,
        #[yaserde(rename = "userdata1", prefix = "tns", default)]
        pub userdata_1: Option<String>,
        #[yaserde(rename = "userdata2", prefix = "tns", default)]
        pub userdata_2: Option<String>,
        #[yaserde(rename = "userdata3", prefix = "tns", default)]
        pub userdata_3: Option<String>,
        #[yaserde(rename = "userdata4", prefix = "tns", default)]
        pub userdata_4: Option<String>,
        #[yaserde(rename = "pin", prefix = "tns", default)]
        pub pin: Option<String>,
        #[yaserde(rename = "imageID", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "brncode", prefix = "tns", default)]
        pub brncode: Option<String>,
        #[yaserde(rename = "renew", prefix = "tns", default)]
        pub renew: Option<String>,
        #[yaserde(rename = "dlvMethod", prefix = "tns", default)]
        pub dlv_method: Option<String>,
        #[yaserde(rename = "denyMCC", prefix = "tns", default)]
        pub deny_mcc: Option<String>,
        #[yaserde(rename = "denySvc", prefix = "tns", default)]
        pub deny_svc: Option<String>,
        #[yaserde(rename = "accType", prefix = "tns", default)]
        pub acc_type: Option<String>,
        #[yaserde(rename = "memo", prefix = "tns", default)]
        pub memo: Option<String>,
        #[yaserde(rename = "memoScope", prefix = "tns", default)]
        pub memo_scope: i32,
        #[yaserde(rename = "memoUser", prefix = "tns", default)]
        pub memo_user: Option<String>,
        #[yaserde(rename = "itemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "dlvTitle", prefix = "tns", default)]
        pub dlv_title: Option<String>,
        #[yaserde(rename = "dlvfirstName", prefix = "tns", default)]
        pub dlvfirst_name: Option<String>,
        #[yaserde(rename = "dlvlastName", prefix = "tns", default)]
        pub dlvlast_name: Option<String>,
        #[yaserde(rename = "dlvaddr1", prefix = "tns", default)]
        pub dlvaddr_1: Option<String>,
        #[yaserde(rename = "dlvaddr2", prefix = "tns", default)]
        pub dlvaddr_2: Option<String>,
        #[yaserde(rename = "dlvaddr3", prefix = "tns", default)]
        pub dlvaddr_3: Option<String>,
        #[yaserde(rename = "dlvcity", prefix = "tns", default)]
        pub dlvcity: Option<String>,
        #[yaserde(rename = "dlvpostcode", prefix = "tns", default)]
        pub dlvpostcode: Option<String>,
        #[yaserde(rename = "dlvcounty", prefix = "tns", default)]
        pub dlvcounty: Option<String>,
        #[yaserde(rename = "dlvcountry", prefix = "tns", default)]
        pub dlvcountry: Option<String>,
        #[yaserde(rename = "dlvtel", prefix = "tns", default)]
        pub dlvtel: Option<String>,
        #[yaserde(rename = "dlvEffDate", prefix = "tns", default)]
        pub dlv_eff_date: Option<String>,
        #[yaserde(rename = "dlvDaysValid", prefix = "tns", default)]
        pub dlv_days_valid: i32,
        #[yaserde(rename = "crdprogram", prefix = "tns", default)]
        pub crdprogram: Option<String>,
        #[yaserde(rename = "crddesign", prefix = "tns", default)]
        pub crddesign: Option<String>,
        #[yaserde(rename = "feeTier", prefix = "tns", default)]
        pub fee_tier: Option<String>,
        #[yaserde(rename = "isoLang", prefix = "tns", default)]
        pub iso_lang: Option<String>,
        #[yaserde(rename = "fundcrdPAN", prefix = "tns", default)]
        pub fundcrd_pan: Option<String>,
        #[yaserde(rename = "fundCrdEffDate", prefix = "tns", default)]
        pub fund_crd_eff_date: Option<String>,
        #[yaserde(rename = "fundCrdExpDate", prefix = "tns", default)]
        pub fund_crd_exp_date: Option<String>,
        #[yaserde(rename = "fundCrdType", prefix = "tns", default)]
        pub fund_crd_type: Option<String>,
        #[yaserde(rename = "fundCrdIssNum", prefix = "tns", default)]
        pub fund_crd_iss_num: i32,
        #[yaserde(rename = "fundCrdCVC", prefix = "tns", default)]
        pub fund_crd_cvc: i32,
        #[yaserde(rename = "svcSrc", prefix = "tns", default)]
        pub svc_src: i32,
        #[yaserde(rename = "svcType", prefix = "tns", default)]
        pub svc_type: i32,
        #[yaserde(rename = "svcStatus", prefix = "tns", default)]
        pub svc_status: i32,
        #[yaserde(rename = "secID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "SmsBalance", prefix = "tns", default)]
        pub sms_balance: i32,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "VanityName", prefix = "tns", default)]
        pub vanity_name: Option<String>,
        #[yaserde(rename = "addr3", prefix = "tns", default)]
        pub addr_3: Option<String>,
        #[yaserde(rename = "CarrierType", prefix = "tns", default)]
        pub carrier_type: Option<String>,
        #[yaserde(rename = "Delv_Code", prefix = "tns", default)]
        pub delv_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Update_Cardholder_DetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateCardholderDetailsResponse {
        #[yaserde(rename = "Ws_Update_Cardholder_DetailsResult", prefix = "tns", default)]
        pub ws_update_cardholder_details_result: CustomerUpdate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerUpdate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CustomerUpdate {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CrdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UnLoad_StatusChange",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUnLoadStatusChange {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "AmtUnLoad", prefix = "tns", default)]
        pub amt_un_load: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UnLoad_StatusChangeResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUnLoadStatusChangeResponse {
        #[yaserde(rename = "Ws_UnLoad_StatusChangeResult", prefix = "tns", default)]
        pub ws_un_load_status_change_result: UnLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Activate_Load",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsActivateLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "ActMethod", prefix = "tns", default)]
        pub act_method: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "ActivateOrNot", prefix = "tns", default)]
        pub activate_or_not: i32,
        #[yaserde(rename = "PANorToken", prefix = "tns", default)]
        pub pa_nor_token: i32,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "SMSBalance", prefix = "tns", default)]
        pub sms_balance: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "BrnCode", prefix = "tns", default)]
        pub brn_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Activate_LoadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsActivateLoadResponse {
        #[yaserde(rename = "Ws_Activate_LoadResult", prefix = "tns", default)]
        pub ws_activate_load_result: ActivateAndLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ActivateAndLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ActivateAndLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BalanceAdjustment",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBalanceAdjustment {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AmtAdjustment", prefix = "tns", default)]
        pub amt_adjustment: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "DebOrCred", prefix = "tns", default)]
        pub deb_or_cred: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "ForcePost", prefix = "tns", default)]
        pub force_post: bool,
        #[yaserde(rename = "ExtCode", prefix = "tns", default)]
        pub ext_code: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BalanceAdjustmentResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsBalanceAdjustmentResponse {
        #[yaserde(rename = "Ws_BalanceAdjustmentResult", default)]
        pub ws_balance_adjustment_result: BalanceAdjust,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "BalanceAdjust")]
    pub struct BalanceAdjust {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "AvlBal", default)]
        pub avl_bal: String,
        #[yaserde(rename = "CurCode", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ItemID", default)]
        pub item_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_ExtendExpiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsExtendExpiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_ExtendExpiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsExtendExpiryResponse {
        #[yaserde(rename = "Ws_ExtendExpiryResult", prefix = "tns", default)]
        pub ws_extend_expiry_result: ExtendExpiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ExtendExpiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ExtendExpiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Transaction_Void",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsTransactionVoid {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "OrgItemId", prefix = "tns", default)]
        pub org_item_id: i64,
        #[yaserde(rename = "Note", prefix = "tns", default)]
        pub note: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Transaction_VoidResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsTransactionVoidResponse {
        #[yaserde(rename = "Ws_Transaction_VoidResult", prefix = "tns", default)]
        pub ws_transaction_void_result: TransactionVoid,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransactionVoid",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct TransactionVoid {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "OrgItemId", prefix = "tns", default)]
        pub org_item_id: i64,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ItemId", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardHolder_Details_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardHolderDetailsEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "secID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "secVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardHolder_Details_EnquiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardHolderDetailsEnquiryResponse {
        #[yaserde(
            rename = "Ws_CardHolder_Details_EnquiryResult",
            prefix = "tns",
            default
        )]
        pub ws_card_holder_details_enquiry_result: CardHolderDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardHolderDetails",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardHolderDetails {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "Tel", prefix = "tns", default)]
        pub tel: Option<String>,
        #[yaserde(rename = "WorkAddrl1", prefix = "tns", default)]
        pub work_addrl_1: Option<String>,
        #[yaserde(rename = "WorkAddrl2", prefix = "tns", default)]
        pub work_addrl_2: Option<String>,
        #[yaserde(rename = "WorkAddrl3", prefix = "tns", default)]
        pub work_addrl_3: Option<String>,
        #[yaserde(rename = "WorkCity", prefix = "tns", default)]
        pub work_city: Option<String>,
        #[yaserde(rename = "WorkPostCode", prefix = "tns", default)]
        pub work_post_code: Option<String>,
        #[yaserde(rename = "WorkCounty", prefix = "tns", default)]
        pub work_county: Option<String>,
        #[yaserde(rename = "WorkCountry", prefix = "tns", default)]
        pub work_country: Option<String>,
        #[yaserde(rename = "WorkTel", prefix = "tns", default)]
        pub work_tel: Option<String>,
        #[yaserde(rename = "EMail", prefix = "tns", default)]
        pub e_mail: Option<String>,
        #[yaserde(rename = "Fax", prefix = "tns", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "POBox", prefix = "tns", default)]
        pub po_box: Option<String>,
        #[yaserde(rename = "MobTel", prefix = "tns", default)]
        pub mob_tel: Option<String>,
        #[yaserde(rename = "MaritalStatus", prefix = "tns", default)]
        pub marital_status: Option<String>,
        #[yaserde(rename = "Sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "AccNo", prefix = "tns", default)]
        pub acc_no: Option<String>,
        #[yaserde(rename = "CrdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "EmbossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "RefuseCheck", prefix = "tns", default)]
        pub refuse_check: Option<String>,
        #[yaserde(rename = "MailShots", prefix = "tns", default)]
        pub mail_shots: Option<String>,
        #[yaserde(rename = "Discret", prefix = "tns", default)]
        pub discret: Option<String>,
        #[yaserde(rename = "UsrData", prefix = "tns", default)]
        pub usr_data: Option<String>,
        #[yaserde(rename = "UsrData1", prefix = "tns", default)]
        pub usr_data_1: Option<String>,
        #[yaserde(rename = "UsrData2", prefix = "tns", default)]
        pub usr_data_2: Option<String>,
        #[yaserde(rename = "UsrData3", prefix = "tns", default)]
        pub usr_data_3: Option<String>,
        #[yaserde(rename = "UsrData4", prefix = "tns", default)]
        pub usr_data_4: Option<String>,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "EffDate", prefix = "tns", default)]
        pub eff_date: Option<String>,
        #[yaserde(rename = "SvcCode", prefix = "tns", default)]
        pub svc_code: Option<String>,
        #[yaserde(rename = "AdditionalNo", prefix = "tns", default)]
        pub additional_no: i32,
        #[yaserde(rename = "DateCreated", prefix = "tns", default)]
        pub date_created: Option<String>,
        #[yaserde(rename = "DateActivated", prefix = "tns", default)]
        pub date_activated: Option<String>,
        #[yaserde(rename = "CrdDesign", prefix = "tns", default)]
        pub crd_design: Option<String>,
        #[yaserde(rename = "PIN", prefix = "tns", default)]
        pub pin: Option<String>,
        #[yaserde(rename = "DlvMethod", prefix = "tns", default)]
        pub dlv_method: Option<String>,
        #[yaserde(rename = "ImageID", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "BrnCode", prefix = "tns", default)]
        pub brn_code: Option<String>,
        #[yaserde(rename = "ReNew", prefix = "tns", default)]
        pub re_new: Option<String>,
        #[yaserde(rename = "DenyMCC", prefix = "tns", default)]
        pub deny_mcc: Option<String>,
        #[yaserde(rename = "DenySVC", prefix = "tns", default)]
        pub deny_svc: Option<String>,
        #[yaserde(rename = "AccType", prefix = "tns", default)]
        pub acc_type: Option<String>,
        #[yaserde(rename = "CVC2", prefix = "tns", default)]
        pub cvc2: Option<String>,
        #[yaserde(rename = "DlvTitle", prefix = "tns", default)]
        pub dlv_title: Option<String>,
        #[yaserde(rename = "DlvFirstName", prefix = "tns", default)]
        pub dlv_first_name: Option<String>,
        #[yaserde(rename = "DlvLastName", prefix = "tns", default)]
        pub dlv_last_name: Option<String>,
        #[yaserde(rename = "DlvAddrL1", prefix = "tns", default)]
        pub dlv_addr_l1: Option<String>,
        #[yaserde(rename = "DlvAddrL2", prefix = "tns", default)]
        pub dlv_addr_l2: Option<String>,
        #[yaserde(rename = "DlvAddrL3", prefix = "tns", default)]
        pub dlv_addr_l3: Option<String>,
        #[yaserde(rename = "DlvCity", prefix = "tns", default)]
        pub dlv_city: Option<String>,
        #[yaserde(rename = "DlvCounty", prefix = "tns", default)]
        pub dlv_county: Option<String>,
        #[yaserde(rename = "DlvCountry", prefix = "tns", default)]
        pub dlv_country: Option<String>,
        #[yaserde(rename = "DlvTel", prefix = "tns", default)]
        pub dlv_tel: Option<String>,
        #[yaserde(rename = "DlvEffDate", prefix = "tns", default)]
        pub dlv_eff_date: Option<String>,
        #[yaserde(rename = "DlvExpDate", prefix = "tns", default)]
        pub dlv_exp_date: Option<String>,
        #[yaserde(rename = "IsoLang", prefix = "tns", default)]
        pub iso_lang: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardHolder_Details_Enquiry_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardHolderDetailsEnquiryV2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "secID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "secVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardHolder_Details_Enquiry_V2Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardHolderDetailsEnquiryV2Response {
        #[yaserde(
            rename = "Ws_CardHolder_Details_Enquiry_V2Result",
            prefix = "tns",
            default
        )]
        pub ws_card_holder_details_enquiry_v2_result: CardHolderDetails2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardHolderDetails2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardHolderDetails2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "Addrl3", prefix = "tns", default)]
        pub addrl_3: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "Tel", prefix = "tns", default)]
        pub tel: Option<String>,
        #[yaserde(rename = "WorkAddrl1", prefix = "tns", default)]
        pub work_addrl_1: Option<String>,
        #[yaserde(rename = "WorkAddrl2", prefix = "tns", default)]
        pub work_addrl_2: Option<String>,
        #[yaserde(rename = "WorkAddrl3", prefix = "tns", default)]
        pub work_addrl_3: Option<String>,
        #[yaserde(rename = "WorkCity", prefix = "tns", default)]
        pub work_city: Option<String>,
        #[yaserde(rename = "WorkPostCode", prefix = "tns", default)]
        pub work_post_code: Option<String>,
        #[yaserde(rename = "WorkCounty", prefix = "tns", default)]
        pub work_county: Option<String>,
        #[yaserde(rename = "WorkCountry", prefix = "tns", default)]
        pub work_country: Option<String>,
        #[yaserde(rename = "WorkTel", prefix = "tns", default)]
        pub work_tel: Option<String>,
        #[yaserde(rename = "EMail", prefix = "tns", default)]
        pub e_mail: Option<String>,
        #[yaserde(rename = "Fax", prefix = "tns", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "POBox", prefix = "tns", default)]
        pub po_box: Option<String>,
        #[yaserde(rename = "MobTel", prefix = "tns", default)]
        pub mob_tel: Option<String>,
        #[yaserde(rename = "MaritalStatus", prefix = "tns", default)]
        pub marital_status: Option<String>,
        #[yaserde(rename = "Sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "CrdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "EmbossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "RefuseCheck", prefix = "tns", default)]
        pub refuse_check: Option<String>,
        #[yaserde(rename = "MailShots", prefix = "tns", default)]
        pub mail_shots: Option<String>,
        #[yaserde(rename = "Discret", prefix = "tns", default)]
        pub discret: Option<String>,
        #[yaserde(rename = "UsrData", prefix = "tns", default)]
        pub usr_data: Option<String>,
        #[yaserde(rename = "UsrData1", prefix = "tns", default)]
        pub usr_data_1: Option<String>,
        #[yaserde(rename = "UsrData2", prefix = "tns", default)]
        pub usr_data_2: Option<String>,
        #[yaserde(rename = "UsrData3", prefix = "tns", default)]
        pub usr_data_3: Option<String>,
        #[yaserde(rename = "UsrData4", prefix = "tns", default)]
        pub usr_data_4: Option<String>,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "EffDate", prefix = "tns", default)]
        pub eff_date: Option<String>,
        #[yaserde(rename = "SvcCode", prefix = "tns", default)]
        pub svc_code: Option<String>,
        #[yaserde(rename = "AdditionalNo", prefix = "tns", default)]
        pub additional_no: Option<String>,
        #[yaserde(rename = "DateCreated", prefix = "tns", default)]
        pub date_created: Option<String>,
        #[yaserde(rename = "DateActivated", prefix = "tns", default)]
        pub date_activated: Option<String>,
        #[yaserde(rename = "CrdDesign", prefix = "tns", default)]
        pub crd_design: Option<String>,
        #[yaserde(rename = "PIN", prefix = "tns", default)]
        pub pin: Option<String>,
        #[yaserde(rename = "DlvMethod", prefix = "tns", default)]
        pub dlv_method: Option<String>,
        #[yaserde(rename = "ImageID", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "BrnCode", prefix = "tns", default)]
        pub brn_code: Option<String>,
        #[yaserde(rename = "ReNew", prefix = "tns", default)]
        pub re_new: Option<String>,
        #[yaserde(rename = "DenyMCC", prefix = "tns", default)]
        pub deny_mcc: Option<String>,
        #[yaserde(rename = "DenySVC", prefix = "tns", default)]
        pub deny_svc: Option<String>,
        #[yaserde(rename = "AccType", prefix = "tns", default)]
        pub acc_type: Option<String>,
        #[yaserde(rename = "CVC2", prefix = "tns", default)]
        pub cvc2: Option<String>,
        #[yaserde(rename = "DlvTitle", prefix = "tns", default)]
        pub dlv_title: Option<String>,
        #[yaserde(rename = "DlvFirstName", prefix = "tns", default)]
        pub dlv_first_name: Option<String>,
        #[yaserde(rename = "DlvLastName", prefix = "tns", default)]
        pub dlv_last_name: Option<String>,
        #[yaserde(rename = "DlvAddrL1", prefix = "tns", default)]
        pub dlv_addr_l1: Option<String>,
        #[yaserde(rename = "DlvAddrL2", prefix = "tns", default)]
        pub dlv_addr_l2: Option<String>,
        #[yaserde(rename = "DlvAddrL3", prefix = "tns", default)]
        pub dlv_addr_l3: Option<String>,
        #[yaserde(rename = "DlvCity", prefix = "tns", default)]
        pub dlv_city: Option<String>,
        #[yaserde(rename = "DlvCounty", prefix = "tns", default)]
        pub dlv_county: Option<String>,
        #[yaserde(rename = "DlvCountry", prefix = "tns", default)]
        pub dlv_country: Option<String>,
        #[yaserde(rename = "DlvTel", prefix = "tns", default)]
        pub dlv_tel: Option<String>,
        #[yaserde(rename = "DlvEffDate", prefix = "tns", default)]
        pub dlv_eff_date: Option<String>,
        #[yaserde(rename = "DlvExpDate", prefix = "tns", default)]
        pub dlv_exp_date: Option<String>,
        #[yaserde(rename = "IsoLang", prefix = "tns", default)]
        pub iso_lang: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "VanityName", prefix = "tns", default)]
        pub vanity_name: Option<String>,
        #[yaserde(rename = "DlvPostcode", prefix = "tns", default)]
        pub dlv_postcode: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Phone_Activation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPhoneActivation {
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "SecurityCode", prefix = "tns", default)]
        pub security_code: Option<String>,
        #[yaserde(rename = "ActivateIfNot", prefix = "tns", default)]
        pub activate_if_not: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Phone_ActivationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPhoneActivationResponse {
        #[yaserde(rename = "Ws_Phone_ActivationResult", prefix = "tns", default)]
        pub ws_phone_activation_result: PhoneActivate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PhoneActivate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PhoneActivate {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
        #[yaserde(rename = "PinBlock", prefix = "tns", default)]
        pub pin_block: Option<String>,
        #[yaserde(rename = "PINStatus", prefix = "tns", default)]
        pub pin_status: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "doc",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Doc {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BulkCreation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBulkCreation {
        #[yaserde(rename = "doc", prefix = "tns", default)]
        pub doc: Option<Doc>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BulkCreationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBulkCreationResponse {
        #[yaserde(rename = "Ws_BulkCreationResult", prefix = "tns", default)]
        pub ws_bulk_creation_result: BulkCards,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Cards",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Cards {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BulkCards",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BulkCards {
        #[yaserde(rename = "CardsCreated", prefix = "tns", default)]
        pub cards_created: i32,
        #[yaserde(rename = "CardsNotCreated", prefix = "tns", default)]
        pub cards_not_created: i32,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "RequestWithError", prefix = "tns", default)]
        pub request_with_error: Option<String>,
        #[yaserde(rename = "Cards", prefix = "tns", default)]
        pub cards: Option<Cards>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Req",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Req {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BulkWalletCreation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBulkWalletCreation {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Req", prefix = "tns", default)]
        pub req: Option<Req>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_BulkWalletCreationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBulkWalletCreationResponse {
        #[yaserde(rename = "Ws_BulkWalletCreationResult", prefix = "tns", default)]
        pub ws_bulk_wallet_creation_result: BulkWallets,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Wallets",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Wallets {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BulkWallets",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BulkWallets {
        #[yaserde(rename = "WalletCreated", prefix = "tns", default)]
        pub wallet_created: i32,
        #[yaserde(rename = "WalletNotCreated", prefix = "tns", default)]
        pub wallet_not_created: i32,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "RequestWithError", prefix = "tns", default)]
        pub request_with_error: Option<String>,
        #[yaserde(rename = "Wallets", prefix = "tns", default)]
        pub wallets: Option<Wallets>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_WebServiceResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsWebServiceResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "Isscode", prefix = "tns", default)]
        pub isscode: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_WebServiceResultResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsWebServiceResultResponse {
        #[yaserde(rename = "Ws_WebServiceResultResult", prefix = "tns", default)]
        pub ws_web_service_result_result: WsResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WsResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Response", prefix = "tns", default)]
        pub response: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Generic_Fees",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGenericFees {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ProcCode", prefix = "tns", default)]
        pub proc_code: Option<String>,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Fee", prefix = "tns", default)]
        pub fee: f64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Generic_FeesResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGenericFeesResponse {
        #[yaserde(rename = "Ws_Generic_FeesResult", prefix = "tns", default)]
        pub ws_generic_fees_result: ApplyFees,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ApplyFees",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ApplyFees {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Fee", prefix = "tns", default)]
        pub fee: f64,
        #[yaserde(rename = "ItemId", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "IsPartial", prefix = "tns", default)]
        pub is_partial: bool,
        #[yaserde(rename = "WaivedoffAmount", prefix = "tns", default)]
        pub waivedoff_amount: f64,
        #[yaserde(rename = "TotalAmount", prefix = "tns", default)]
        pub total_amount: f64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_BalEnq",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardBalEnq {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "AccNo", prefix = "tns", default)]
        pub acc_no: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "SecId", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_BalEnqResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardBalEnqResponse {
        #[yaserde(rename = "Ws_Card_BalEnqResult", prefix = "tns", default)]
        pub ws_card_bal_enq_result: CardEnquiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardEnquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Logical_Expiry", prefix = "tns", default)]
        pub logical_expiry: Option<String>,
        #[yaserde(rename = "Physical_Expiry", prefix = "tns", default)]
        pub physical_expiry: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "EmbossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
        #[yaserde(rename = "Scheme", prefix = "tns", default)]
        pub scheme: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WS_PinControl",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPinControl {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "locDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "locTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Func", prefix = "tns", default)]
        pub func: Option<String>,
        #[yaserde(rename = "CurrentPin", prefix = "tns", default)]
        pub current_pin: Option<String>,
        #[yaserde(rename = "NewPin", prefix = "tns", default)]
        pub new_pin: Option<String>,
        #[yaserde(rename = "ConfirmPin", prefix = "tns", default)]
        pub confirm_pin: Option<String>,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
        #[yaserde(rename = "ExtAPICardID", prefix = "tns", default)]
        pub ext_api_card_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WS_PinControlResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPinControlResponse {
        #[yaserde(rename = "WS_PinControlResult", prefix = "tns", default)]
        pub ws_pin_control_result: PinControlResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PinControlResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PinControlResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "CurrentPin", prefix = "tns", default)]
        pub current_pin: Option<String>,
        #[yaserde(rename = "NewPin", prefix = "tns", default)]
        pub new_pin: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PINStatus", prefix = "tns", default)]
        pub pin_status: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CreateCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCreateCard {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "Addrl3", prefix = "tns", default)]
        pub addrl_3: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "Mobile", prefix = "tns", default)]
        pub mobile: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: String,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "CreateImage", prefix = "tns", default)]
        pub create_image: i32,
        #[yaserde(rename = "CreateType", prefix = "tns", default)]
        pub create_type: i32,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "ActivateNow", prefix = "tns", default)]
        pub activate_now: i32,
        #[yaserde(rename = "Source_desc", prefix = "tns", default)]
        pub source_desc: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CardName", prefix = "tns", default)]
        pub card_name: Option<String>,
        #[yaserde(rename = "LimitsGroup", prefix = "tns", default)]
        pub limits_group: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "ProductRef", prefix = "tns", default)]
        pub product_ref: Option<String>,
        #[yaserde(rename = "CarrierType", prefix = "tns", default)]
        pub carrier_type: Option<String>,
        #[yaserde(rename = "Fulfil1", prefix = "tns", default)]
        pub fulfil_1: Option<String>,
        #[yaserde(rename = "Fulfil2", prefix = "tns", default)]
        pub fulfil_2: Option<String>,
        #[yaserde(rename = "DelvMethod", prefix = "tns", default)]
        pub delv_method: Option<String>,
        #[yaserde(rename = "ThermalLine1", prefix = "tns", default)]
        pub thermal_line_1: Option<String>,
        #[yaserde(rename = "ThermalLine2", prefix = "tns", default)]
        pub thermal_line_2: Option<String>,
        #[yaserde(rename = "EmbossLine4", prefix = "tns", default)]
        pub emboss_line_4: Option<String>,
        #[yaserde(rename = "ImageId", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "LogoFrontId", prefix = "tns", default)]
        pub logo_front_id: Option<String>,
        #[yaserde(rename = "LogoBackId", prefix = "tns", default)]
        pub logo_back_id: Option<String>,
        #[yaserde(rename = "Replacement", prefix = "tns", default)]
        pub replacement: bool,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "PrimaryToken", prefix = "tns", default)]
        pub primary_token: Option<String>,
        #[yaserde(rename = "Delv_AddrL1", prefix = "tns", default)]
        pub delv_addr_l1: Option<String>,
        #[yaserde(rename = "Delv_AddrL2", prefix = "tns", default)]
        pub delv_addr_l2: Option<String>,
        #[yaserde(rename = "Delv_AddrL3", prefix = "tns", default)]
        pub delv_addr_l3: Option<String>,
        #[yaserde(rename = "Delv_City", prefix = "tns", default)]
        pub delv_city: Option<String>,
        #[yaserde(rename = "Delv_County", prefix = "tns", default)]
        pub delv_county: Option<String>,
        #[yaserde(rename = "Delv_PostCode", prefix = "tns", default)]
        pub delv_post_code: Option<String>,
        #[yaserde(rename = "Delv_Country", prefix = "tns", default)]
        pub delv_country: Option<String>,
        #[yaserde(rename = "Delv_Code", prefix = "tns", default)]
        pub delv_code: Option<String>,
        #[yaserde(rename = "Lang", prefix = "tns", default)]
        pub lang: Option<String>,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", prefix = "tns", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "CardManufacturer", prefix = "tns", default)]
        pub card_manufacturer: Option<String>,
        #[yaserde(rename = "CoBrand", prefix = "tns", default)]
        pub co_brand: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ExternalAuth", prefix = "tns", default)]
        pub external_auth: Option<String>,
        #[yaserde(rename = "LinkageGroup", prefix = "tns", default)]
        pub linkage_group: Option<String>,
        #[yaserde(rename = "VanityName", prefix = "tns", default)]
        pub vanity_name: Option<String>,
        #[yaserde(rename = "PBlock", prefix = "tns", default)]
        pub p_block: Option<String>,
        #[yaserde(rename = "PINMailer", prefix = "tns", default)]
        pub pin_mailer: Option<String>,
        #[yaserde(rename = "FxGroup", prefix = "tns", default)]
        pub fx_group: Option<String>,
        #[yaserde(rename = "Email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "MailOrSMS", prefix = "tns", default)]
        pub mail_or_sms: Option<String>,
        #[yaserde(rename = "AuthCalendarGroup", prefix = "tns", default)]
        pub auth_calendar_group: Option<String>,
        #[yaserde(rename = "Quantity", prefix = "tns", default)]
        pub quantity: Option<String>,
        #[yaserde(rename = "LoadToken", prefix = "tns", default)]
        pub load_token: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
        #[yaserde(rename = "BlackList", prefix = "tns", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "WhiteList", prefix = "tns", default)]
        pub white_list: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", prefix = "tns", default)]
        pub payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CreateCardResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsCreateCardResponse {
        #[yaserde(rename = "Ws_CreateCardResult", default)]
        pub ws_create_card_result: VirtualCards,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "VirtualCards")]
    pub struct VirtualCards {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ExternalRef", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemID", default)]
        pub item_id: i64,
        #[yaserde(rename = "ClientCode", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LoadValue", default)]
        pub load_value: String,
        #[yaserde(rename = "IsLive", default)]
        pub is_live: bool,
        #[yaserde(rename = "StartDate", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "ExpDate", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CVV", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "MaskedPAN", default)]
        pub masked_pan: Option<String>,
        #[yaserde(rename = "Image", default)]
        pub image: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CreateWallet",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCreateWallet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "Addrl3", prefix = "tns", default)]
        pub addrl_3: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "Mobile", prefix = "tns", default)]
        pub mobile: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "CreateImage", prefix = "tns", default)]
        pub create_image: i32,
        #[yaserde(rename = "CreateType", prefix = "tns", default)]
        pub create_type: i32,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "ActivateNow", prefix = "tns", default)]
        pub activate_now: i32,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CardName", prefix = "tns", default)]
        pub card_name: Option<String>,
        #[yaserde(rename = "LimitsGroup", prefix = "tns", default)]
        pub limits_group: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "ProductRef", prefix = "tns", default)]
        pub product_ref: Option<String>,
        #[yaserde(rename = "CarrierType", prefix = "tns", default)]
        pub carrier_type: Option<String>,
        #[yaserde(rename = "Fulfil1", prefix = "tns", default)]
        pub fulfil_1: Option<String>,
        #[yaserde(rename = "Fulfil2", prefix = "tns", default)]
        pub fulfil_2: Option<String>,
        #[yaserde(rename = "DelvMethod", prefix = "tns", default)]
        pub delv_method: Option<String>,
        #[yaserde(rename = "ThermalLine1", prefix = "tns", default)]
        pub thermal_line_1: Option<String>,
        #[yaserde(rename = "ThermalLine2", prefix = "tns", default)]
        pub thermal_line_2: Option<String>,
        #[yaserde(rename = "EmbossLine4", prefix = "tns", default)]
        pub emboss_line_4: Option<String>,
        #[yaserde(rename = "ImageId", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "LogoFrontId", prefix = "tns", default)]
        pub logo_front_id: Option<String>,
        #[yaserde(rename = "LogoBackId", prefix = "tns", default)]
        pub logo_back_id: Option<String>,
        #[yaserde(rename = "Replacement", prefix = "tns", default)]
        pub replacement: bool,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "PrimaryToken", prefix = "tns", default)]
        pub primary_token: Option<String>,
        #[yaserde(rename = "Delv_AddrL1", prefix = "tns", default)]
        pub delv_addr_l1: Option<String>,
        #[yaserde(rename = "Delv_AddrL2", prefix = "tns", default)]
        pub delv_addr_l2: Option<String>,
        #[yaserde(rename = "Delv_AddrL3", prefix = "tns", default)]
        pub delv_addr_l3: Option<String>,
        #[yaserde(rename = "Delv_City", prefix = "tns", default)]
        pub delv_city: Option<String>,
        #[yaserde(rename = "Delv_County", prefix = "tns", default)]
        pub delv_county: Option<String>,
        #[yaserde(rename = "Delv_PostCode", prefix = "tns", default)]
        pub delv_post_code: Option<String>,
        #[yaserde(rename = "Delv_Country", prefix = "tns", default)]
        pub delv_country: Option<String>,
        #[yaserde(rename = "Delv_Code", prefix = "tns", default)]
        pub delv_code: Option<String>,
        #[yaserde(rename = "Lang", prefix = "tns", default)]
        pub lang: Option<String>,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", prefix = "tns", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "CardManufacturer", prefix = "tns", default)]
        pub card_manufacturer: Option<String>,
        #[yaserde(rename = "CoBrand", prefix = "tns", default)]
        pub co_brand: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ExternalAuth", prefix = "tns", default)]
        pub external_auth: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
        #[yaserde(rename = "BlackList", prefix = "tns", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "WhiteList", prefix = "tns", default)]
        pub white_list: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", prefix = "tns", default)]
        pub payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CreateWalletResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCreateWalletResponse {
        #[yaserde(rename = "Ws_CreateWalletResult", prefix = "tns", default)]
        pub ws_create_wallet_result: Wallet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Wallet",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Wallet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "MaskedPAN", prefix = "tns", default)]
        pub masked_pan: Option<String>,
        #[yaserde(rename = "EWALLET", prefix = "tns", default)]
        pub ewallet: Option<Ewallet>,
        #[yaserde(rename = "Image", prefix = "tns", default)]
        pub image: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Regenerate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsRegenerate {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "RegenType", prefix = "tns", default)]
        pub regen_type: i32,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: i32,
        #[yaserde(rename = "Sms_Content", prefix = "tns", default)]
        pub sms_content: i32,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "MailOrSMS", prefix = "tns", default)]
        pub mail_or_sms: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_RegenerateResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsRegenerateResponse {
        #[yaserde(rename = "Ws_RegenerateResult", prefix = "tns", default)]
        pub ws_regenerate_result: Regenerate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Regenerate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Regenerate {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "Image", prefix = "tns", default)]
        pub image: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Convert_Card",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsConvertCard {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ConvertDate", prefix = "tns", default)]
        pub convert_date: Option<String>,
        #[yaserde(rename = "Apply_Fee", prefix = "tns", default)]
        pub apply_fee: i32,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Convert_CardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsConvertCardResponse {
        #[yaserde(rename = "Ws_Convert_CardResult", prefix = "tns", default)]
        pub ws_convert_card_result: ConvertCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ConvertCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ConvertCard {
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ConvertDate", prefix = "tns", default)]
        pub convert_date: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Change_Groups",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsChangeGroups {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardSelector", prefix = "tns", default)]
        pub card_selector: i32,
        #[yaserde(rename = "CardSelectorValue", prefix = "tns", default)]
        pub card_selector_value: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "LimitsGroup", prefix = "tns", default)]
        pub limits_group: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", prefix = "tns", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "CurrentLimitsGroup", prefix = "tns", default)]
        pub current_limits_group: Option<String>,
        #[yaserde(rename = "CurrentMCCGroup", prefix = "tns", default)]
        pub current_mcc_group: Option<String>,
        #[yaserde(rename = "CurrentPERMSGroup", prefix = "tns", default)]
        pub current_perms_group: Option<String>,
        #[yaserde(rename = "CurrentFeeGroup", prefix = "tns", default)]
        pub current_fee_group: Option<String>,
        #[yaserde(rename = "CurrentSchedFeeGroup", prefix = "tns", default)]
        pub current_sched_fee_group: Option<String>,
        #[yaserde(rename = "CurrentWSFeeGroup", prefix = "tns", default)]
        pub current_ws_fee_group: Option<String>,
        #[yaserde(rename = "LinkageGroup", prefix = "tns", default)]
        pub linkage_group: Option<String>,
        #[yaserde(rename = "CurrentLinkageGroup", prefix = "tns", default)]
        pub current_linkage_group: Option<String>,
        #[yaserde(rename = "FXGroup", prefix = "tns", default)]
        pub fx_group: Option<String>,
        #[yaserde(rename = "CurrentFXGroup", prefix = "tns", default)]
        pub current_fx_group: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", prefix = "tns", default)]
        pub payment_token_usage_group: Option<String>,
        #[yaserde(rename = "CurrentPaymentTokenUsageGroup", prefix = "tns", default)]
        pub current_payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Change_GroupsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsChangeGroupsResponse {
        #[yaserde(rename = "Ws_Change_GroupsResult", prefix = "tns", default)]
        pub ws_change_groups_result: GroupChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GroupChange",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GroupChange {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardSelector", prefix = "tns", default)]
        pub card_selector: i32,
        #[yaserde(rename = "CardSelectorValue", prefix = "tns", default)]
        pub card_selector_value: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "Cards", prefix = "tns", default)]
        pub cards: Option<ArrayOfCardDesc>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardDesc",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardDesc {
        #[yaserde(rename = "CardDesc", prefix = "tns", default)]
        pub card_desc: Vec<CardDesc>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardDesc",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardDesc {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurrentLimitsGroup", prefix = "tns", default)]
        pub current_limits_group: Option<String>,
        #[yaserde(rename = "CurrentMCCGroup", prefix = "tns", default)]
        pub current_mcc_group: Option<String>,
        #[yaserde(rename = "CurrentPERMSGroup", prefix = "tns", default)]
        pub current_perms_group: Option<String>,
        #[yaserde(rename = "CurrentFeeGroup", prefix = "tns", default)]
        pub current_fee_group: Option<String>,
        #[yaserde(rename = "CurrentSchedFeeGroup", prefix = "tns", default)]
        pub current_sched_fee_group: Option<String>,
        #[yaserde(rename = "CurrentWSFeeGroup", prefix = "tns", default)]
        pub current_ws_fee_group: Option<String>,
        #[yaserde(rename = "CurrentLinkageGroup", prefix = "tns", default)]
        pub current_linkage_group: Option<String>,
        #[yaserde(rename = "CurrentFXGroup", prefix = "tns", default)]
        pub current_fx_group: Option<String>,
        #[yaserde(rename = "CurrentPaymentTokenUsageGroup", prefix = "tns", default)]
        pub current_payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Check",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCheck {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CheckResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCheckResponse {
        #[yaserde(rename = "Ws_CheckResult", prefix = "tns", default)]
        pub ws_check_result: Check,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Check",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Check {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Simple_Check",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSimpleCheck {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Simple_CheckResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSimpleCheckResponse {
        #[yaserde(rename = "Ws_Simple_CheckResult", prefix = "tns", default)]
        pub ws_simple_check_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Client_Fx",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsClientFx {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "GroupFxID", prefix = "tns", default)]
        pub group_fx_id: i32,
        #[yaserde(rename = "Rates", prefix = "tns", default)]
        pub rates: Option<ArrayOfRates>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfRates",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfRates {
        #[yaserde(rename = "Rates", prefix = "tns", default)]
        pub rates: Vec<Rates>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Rates",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Rates {
        #[yaserde(rename = "SrcCurCode", prefix = "tns", default)]
        pub src_cur_code: Option<String>,
        #[yaserde(rename = "DesCurCode", prefix = "tns", default)]
        pub des_cur_code: Option<String>,
        #[yaserde(rename = "BuyRate", prefix = "tns", default)]
        pub buy_rate: f64,
        #[yaserde(rename = "SellRate", prefix = "tns", default)]
        pub sell_rate: f64,
        #[yaserde(rename = "MidRate", prefix = "tns", default)]
        pub mid_rate: f64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Client_FxResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsClientFxResponse {
        #[yaserde(rename = "Ws_Client_FxResult", prefix = "tns", default)]
        pub ws_client_fx_result: FxRates,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "FxRates",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct FxRates {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "GroupFxID", prefix = "tns", default)]
        pub group_fx_id: i32,
        #[yaserde(rename = "NoOfInvalidRates", prefix = "tns", default)]
        pub no_of_invalid_rates: i32,
        #[yaserde(rename = "InvalidRates", prefix = "tns", default)]
        pub invalid_rates: Option<ArrayOfInvalidRate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfInvalidRate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfInvalidRate {
        #[yaserde(rename = "InvalidRate", prefix = "tns", default)]
        pub invalid_rate: Vec<InvalidRate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "InvalidRate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct InvalidRate {
        #[yaserde(rename = "RateNo", prefix = "tns", default)]
        pub rate_no: i32,
        #[yaserde(rename = "SrcCurCode", prefix = "tns", default)]
        pub src_cur_code: Option<String>,
        #[yaserde(rename = "DesCurCode", prefix = "tns", default)]
        pub des_cur_code: Option<String>,
        #[yaserde(rename = "BuyRate", prefix = "tns", default)]
        pub buy_rate: f64,
        #[yaserde(rename = "SellRate", prefix = "tns", default)]
        pub sell_rate: f64,
        #[yaserde(rename = "MidRate", prefix = "tns", default)]
        pub mid_rate: f64,
        #[yaserde(rename = "ErrorCode", prefix = "tns", default)]
        pub error_code: Option<String>,
        #[yaserde(rename = "ErrorDescription", prefix = "tns", default)]
        pub error_description: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Link_Cards",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsLinkCards {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "NewPAN", prefix = "tns", default)]
        pub new_pan: Option<String>,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: Option<String>,
        #[yaserde(rename = "LinkageGroup", prefix = "tns", default)]
        pub linkage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Link_CardsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsLinkCardsResponse {
        #[yaserde(rename = "Ws_Link_CardsResult", prefix = "tns", default)]
        pub ws_link_cards_result: LinkCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LinkCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct LinkCard {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "NewPublicToken", prefix = "tns", default)]
        pub new_public_token: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "SecondaryCards", prefix = "tns", default)]
        pub secondary_cards: Option<ArrayOfSecondaryCard>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfSecondaryCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfSecondaryCard {
        #[yaserde(rename = "SecondaryCard", prefix = "tns", default)]
        pub secondary_card: Vec<SecondaryCard>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SecondaryCard",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct SecondaryCard {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Group",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListGroup {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "GroupType", prefix = "tns", default)]
        pub group_type: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_GroupResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListGroupResponse {
        #[yaserde(rename = "Ws_List_GroupResult", prefix = "tns", default)]
        pub ws_list_group_result: GroupList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GroupList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GroupList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "GroupType", prefix = "tns", default)]
        pub group_type: i32,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "GroupInfo", prefix = "tns", default)]
        pub group_info: Option<ArrayOfGroupListInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfGroupListInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfGroupListInfo {
        #[yaserde(rename = "GroupListInfo", prefix = "tns", default)]
        pub group_list_info: Vec<GroupListInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GroupListInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct GroupListInfo {
        #[yaserde(rename = "GroupCode", prefix = "tns", default)]
        pub group_code: Option<String>,
        #[yaserde(rename = "GroupDesc", prefix = "tns", default)]
        pub group_desc: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Products",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListProducts {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_ProductsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListProductsResponse {
        #[yaserde(rename = "Ws_List_ProductsResult", prefix = "tns", default)]
        pub ws_list_products_result: ProductList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ProductList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ProductList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ProductInfo", prefix = "tns", default)]
        pub product_info: Option<ArrayOfProductListInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfProductListInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfProductListInfo {
        #[yaserde(rename = "ProductListInfo", prefix = "tns", default)]
        pub product_list_info: Vec<ProductListInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ProductListInfo",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ProductListInfo {
        #[yaserde(rename = "ProductID", prefix = "tns", default)]
        pub product_id: Option<String>,
        #[yaserde(rename = "ProductName", prefix = "tns", default)]
        pub product_name: Option<String>,
        #[yaserde(rename = "ProductDesc", prefix = "tns", default)]
        pub product_desc: Option<String>,
        #[yaserde(rename = "ProgramID", prefix = "tns", default)]
        pub program_id: Option<String>,
        #[yaserde(rename = "Currency", prefix = "tns", default)]
        pub currency: Option<String>,
        #[yaserde(rename = "CrdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "GroupLimitCode", prefix = "tns", default)]
        pub group_limit_code: Option<String>,
        #[yaserde(rename = "GroupFeeTranCode", prefix = "tns", default)]
        pub group_fee_tran_code: Option<String>,
        #[yaserde(rename = "GroupFeeMasterCode", prefix = "tns", default)]
        pub group_fee_master_code: Option<String>,
        #[yaserde(rename = "GroupFeeWebCode", prefix = "tns", default)]
        pub group_fee_web_code: Option<String>,
        #[yaserde(rename = "GroupMCCCode", prefix = "tns", default)]
        pub group_mcc_code: Option<String>,
        #[yaserde(rename = "GroupUsageCode", prefix = "tns", default)]
        pub group_usage_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GetCardRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetCardRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GetCardRequestResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetCardRequestResponse {
        #[yaserde(rename = "Ws_GetCardRequestResult", prefix = "tns", default)]
        pub ws_get_card_request_result: CardRequestResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardRequestResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardRequestResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "UID", prefix = "tns", default)]
        pub uid: Option<String>,
        #[yaserde(rename = "ProductRef", prefix = "tns", default)]
        pub product_ref: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "ProductionDate", prefix = "tns", default)]
        pub production_date: Option<String>,
        #[yaserde(rename = "ProductionError", prefix = "tns", default)]
        pub production_error: Option<String>,
        #[yaserde(rename = "CardHolderName", prefix = "tns", default)]
        pub card_holder_name: Option<String>,
        #[yaserde(rename = "CardRequestStatus", prefix = "tns", default)]
        pub card_request_status: Option<String>,
        #[yaserde(rename = "CreationDate", prefix = "tns", default)]
        pub creation_date: Option<String>,
        #[yaserde(rename = "CardProductionStatus", prefix = "tns", default)]
        pub card_production_status: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "MaskedPAN", prefix = "tns", default)]
        pub masked_pan: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GetCardRequestStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetCardRequestStatus {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GetCardRequestStatusResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetCardRequestStatusResponse {
        #[yaserde(rename = "Ws_GetCardRequestStatusResult", prefix = "tns", default)]
        pub ws_get_card_request_status_result: CardRequestStatus,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardRequestStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardRequestStatus {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardRequestStatus", prefix = "tns", default)]
        pub card_request_status: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardAcceptorWhiteList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardAcceptorWhiteList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "WhiteList", prefix = "tns", default)]
        pub white_list: Option<String>,
        #[yaserde(rename = "CardAcceptors", prefix = "tns", default)]
        pub card_acceptors: Option<ArrayOfCardAcceptorModifier>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardAcceptorModifier",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardAcceptorModifier {
        #[yaserde(rename = "CardAcceptorModifier", prefix = "tns", default)]
        pub card_acceptor_modifier: Vec<CardAcceptorModifier>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardAcceptorModifier",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardAcceptorModifier {
        #[yaserde(rename = "CardAcceptorID", prefix = "tns", default)]
        pub card_acceptor_id: Option<String>,
        #[yaserde(rename = "Action", prefix = "tns", default)]
        pub action: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardAcceptorWhiteListResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardAcceptorWhiteListResponse {
        #[yaserde(rename = "Ws_CardAcceptorWhiteListResult", prefix = "tns", default)]
        pub ws_card_acceptor_white_list_result: CardAcceptorWhiteList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardAcceptorWhiteList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardAcceptorWhiteList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "WhiteList", prefix = "tns", default)]
        pub white_list: Option<String>,
        #[yaserde(rename = "NoOfInvalidCardAcceptors", prefix = "tns", default)]
        pub no_of_invalid_card_acceptors: i32,
        #[yaserde(rename = "InvalidCardAcceptors", prefix = "tns", default)]
        pub invalid_card_acceptors: Option<ArrayOfInvalidCardAcceptor>,
        #[yaserde(rename = "CardWAcceptors", prefix = "tns", default)]
        pub card_w_acceptors: Option<ArrayOfCardWAcceptor>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfInvalidCardAcceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfInvalidCardAcceptor {
        #[yaserde(rename = "InvalidCardAcceptor", prefix = "tns", default)]
        pub invalid_card_acceptor: Vec<InvalidCardAcceptor>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "InvalidCardAcceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct InvalidCardAcceptor {
        #[yaserde(rename = "AcceptorNo", prefix = "tns", default)]
        pub acceptor_no: i32,
        #[yaserde(rename = "CardAcceptorID", prefix = "tns", default)]
        pub card_acceptor_id: Option<String>,
        #[yaserde(rename = "ErrorCode", prefix = "tns", default)]
        pub error_code: Option<String>,
        #[yaserde(rename = "ErrorDescription", prefix = "tns", default)]
        pub error_description: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardWAcceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardWAcceptor {
        #[yaserde(rename = "CardWAcceptor", prefix = "tns", default)]
        pub card_w_acceptor: Vec<CardWAcceptor>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardWAcceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardWAcceptor {
        #[yaserde(rename = "AcceptorNo", prefix = "tns", default)]
        pub acceptor_no: i32,
        #[yaserde(rename = "CardAcceptorID", prefix = "tns", default)]
        pub card_acceptor_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardAcceptorBlackList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardAcceptorBlackList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "BlackList", prefix = "tns", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "CardAcceptors", prefix = "tns", default)]
        pub card_acceptors: Option<ArrayOfCardAcceptorModifier>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CardAcceptorBlackListResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardAcceptorBlackListResponse {
        #[yaserde(rename = "Ws_CardAcceptorBlackListResult", prefix = "tns", default)]
        pub ws_card_acceptor_black_list_result: CardAcceptorBlackList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardAcceptorBlackList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardAcceptorBlackList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "BlackList", prefix = "tns", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "NoOfInvalidCardAcceptors", prefix = "tns", default)]
        pub no_of_invalid_card_acceptors: i32,
        #[yaserde(rename = "InvalidCardAcceptors", prefix = "tns", default)]
        pub invalid_card_acceptors: Option<ArrayOfInvalidCardAcceptor>,
        #[yaserde(rename = "CardAcceptors", prefix = "tns", default)]
        pub card_acceptors: Option<ArrayOfCardAcceptor>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardAcceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardAcceptor {
        #[yaserde(rename = "CardAcceptor", prefix = "tns", default)]
        pub card_acceptor: Vec<CardAcceptor>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardAcceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardAcceptor {
        #[yaserde(rename = "AcceptorNo", prefix = "tns", default)]
        pub acceptor_no: i32,
        #[yaserde(rename = "CardAcceptorID", prefix = "tns", default)]
        pub card_acceptor_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_SendMessage",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSendMessage {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "EventId", prefix = "tns", default)]
        pub event_id: i32,
        #[yaserde(rename = "MailOrSMS", prefix = "tns", default)]
        pub mail_or_sms: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_SendMessageResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSendMessageResponse {
        #[yaserde(rename = "Ws_SendMessageResult", prefix = "tns", default)]
        pub ws_send_message_result: AlertResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AlertResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct AlertResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_MVCLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsMVCLoad {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "MVCToken", prefix = "tns", default)]
        pub mvc_token: Option<String>,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: Option<String>,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_MVCLoadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsMVCLoadResponse {
        #[yaserde(rename = "Ws_MVCLoadResult", prefix = "tns", default)]
        pub ws_mvc_load_result: Mvcload,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "MVCLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Mvcload {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "MVCToken", prefix = "tns", default)]
        pub mvc_token: Option<String>,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Pending_Fees",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListPendingFees {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "FeeProcessingCode", prefix = "tns", default)]
        pub fee_processing_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Pending_FeesResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListPendingFeesResponse {
        #[yaserde(rename = "Ws_List_Pending_FeesResult", prefix = "tns", default)]
        pub ws_list_pending_fees_result: PendingFees,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PendingFees",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PendingFees {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Fees", prefix = "tns", default)]
        pub fees: Option<ArrayOfFee>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfFee",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfFee {
        #[yaserde(rename = "Fee", prefix = "tns", default)]
        pub fee: Vec<Fee>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Fee",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Fee {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "PostDate", prefix = "tns", default)]
        pub post_date: Option<String>,
        #[yaserde(rename = "TransDate", prefix = "tns", default)]
        pub trans_date: Option<String>,
        #[yaserde(rename = "ProcCode", prefix = "tns", default)]
        pub proc_code: Option<String>,
        #[yaserde(rename = "ActualAmt", prefix = "tns", default)]
        pub actual_amt: f64,
        #[yaserde(rename = "AmtTaken", prefix = "tns", default)]
        pub amt_taken: f64,
        #[yaserde(rename = "RemainingAmt", prefix = "tns", default)]
        pub remaining_amt: f64,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "PartialAllowed", prefix = "tns", default)]
        pub partial_allowed: bool,
        #[yaserde(rename = "Collected", prefix = "tns", default)]
        pub collected: bool,
        #[yaserde(rename = "PendingFeesEnabled", prefix = "tns", default)]
        pub pending_fees_enabled: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_WebServiceResult_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsWebServiceResultV2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_WebServiceResult_V2Response",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsWebServiceResultV2Response {
        #[yaserde(rename = "Ws_WebServiceResult_V2Result", default)]
        pub ws_web_service_result_v2_result: Wsresult2,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    pub struct Response {
        #[yaserde(rename = "Placeholder")]
        pub placeholder: String,
        #[yaserde(attribute)]
        pub xmlns: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    pub struct ResponseSent {
        #[yaserde(rename = "Response", default)]
        pub response: Response,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "WSResult2")]
    pub struct Wsresult2 {
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "WebMethod", default)]
        pub web_method: Option<String>,
        #[yaserde(rename = "ResponseSent", default)]
        pub response_sent: Option<ResponseSent>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Get_Passcode",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetPasscode {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Get_PasscodeResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetPasscodeResponse {
        #[yaserde(rename = "Ws_Get_PasscodeResult", prefix = "tns", default)]
        pub ws_get_passcode_result: PassCode,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PassCode",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PassCode {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Get_Card_ExpireSoon",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetCardExpireSoon {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Get_Card_ExpireSoonResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGetCardExpireSoonResponse {
        #[yaserde(rename = "Ws_Get_Card_ExpireSoonResult", prefix = "tns", default)]
        pub ws_get_card_expire_soon_result: Option<CardResult>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "cardList", prefix = "tns", default)]
        pub card_list: Option<ArrayOfCardDetails>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCard_Details",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardDetails {
        #[yaserde(rename = "Card_Details", prefix = "tns", default)]
        pub card_details: Vec<CardDetails>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Card_Details",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardDetails {
        #[yaserde(rename = "Scheme", prefix = "tns", default)]
        pub scheme: Option<String>,
        #[yaserde(rename = "Product", prefix = "tns", default)]
        pub product: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Emboss_Name", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "ExpiryDate", prefix = "tns", default)]
        pub expiry_date: Option<String>,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
        #[yaserde(rename = "ActivationDate", prefix = "tns", default)]
        pub activation_date: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Send_CardFiles",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSendCardFiles {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: Option<String>,
        #[yaserde(rename = "SFTP", prefix = "tns", default)]
        pub sftp: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Send_CardFilesResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSendCardFilesResponse {
        #[yaserde(rename = "Ws_Send_CardFilesResult", prefix = "tns", default)]
        pub ws_send_card_files_result: CardParams,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Card_Params",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardParams {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: Option<String>,
        #[yaserde(rename = "SFTP", prefix = "tns", default)]
        pub sftp: Option<String>,
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_SafeReports",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSafeReports {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "flag", prefix = "tns", default)]
        pub flag: Option<String>,
        #[yaserde(rename = "TransactionID", prefix = "tns", default)]
        pub transaction_id: Option<String>,
        #[yaserde(rename = "FraudTypeCodeID", prefix = "tns", default)]
        pub fraud_type_code_id: Option<String>,
        #[yaserde(rename = "SubFraudTypeCodeID", prefix = "tns", default)]
        pub sub_fraud_type_code_id: Option<String>,
        #[yaserde(rename = "IsChargedBack", prefix = "tns", default)]
        pub is_charged_back: bool,
        #[yaserde(rename = "IsAccountClosed", prefix = "tns", default)]
        pub is_account_closed: bool,
        #[yaserde(rename = "IssuerId", prefix = "tns", default)]
        pub issuer_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_SafeReportsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSafeReportsResponse {
        #[yaserde(rename = "Ws_SafeReportsResult", prefix = "tns", default)]
        pub ws_safe_reports_result: SafeReport,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SafeReport",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct SafeReport {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "flag", prefix = "tns", default)]
        pub flag: Option<String>,
        #[yaserde(rename = "TransactionID", prefix = "tns", default)]
        pub transaction_id: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_RegenerateWallet",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsRegenerateWallet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "RegenType", prefix = "tns", default)]
        pub regen_type: i32,
        #[yaserde(rename = "Sms_Required", prefix = "tns", default)]
        pub sms_required: i32,
        #[yaserde(rename = "Sms_Content", prefix = "tns", default)]
        pub sms_content: i32,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "MailOrSMS", prefix = "tns", default)]
        pub mail_or_sms: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_RegenerateWalletResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsRegenerateWalletResponse {
        #[yaserde(rename = "Ws_RegenerateWalletResult", prefix = "tns", default)]
        pub ws_regenerate_wallet_result: RegenerateWallet,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RegenerateWallet",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct RegenerateWallet {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "EWALLET", prefix = "tns", default)]
        pub ewallet: Option<Ewallet>,
        #[yaserde(rename = "Image", prefix = "tns", default)]
        pub image: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UpdateLoadSource",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateLoadSource {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ItemId", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_UpdateLoadSourceResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateLoadSourceResponse {
        #[yaserde(rename = "Ws_UpdateLoadSourceResult", prefix = "tns", default)]
        pub ws_update_load_source_result: UpdateLoadSrc,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateLoadSrc",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct UpdateLoadSrc {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_MVCUnload",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsMVCUnload {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "MVCToken", prefix = "tns", default)]
        pub mvc_token: Option<String>,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: Option<String>,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_MVCUnloadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsMVCUnloadResponse {
        #[yaserde(rename = "Ws_MVCUnloadResult", prefix = "tns", default)]
        pub ws_mvc_unload_result: Mvcload,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Activate_MVCLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsActivateMVCLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "ActMethod", prefix = "tns", default)]
        pub act_method: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "MVCPAN", prefix = "tns", default)]
        pub mvcpan: Option<String>,
        #[yaserde(rename = "MVCToken", prefix = "tns", default)]
        pub mvc_token: Option<String>,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
        #[yaserde(rename = "BrnCode", prefix = "tns", default)]
        pub brn_code: Option<String>,
        #[yaserde(rename = "ActivateOrNot", prefix = "tns", default)]
        pub activate_or_not: i32,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SMSBalance", prefix = "tns", default)]
        pub sms_balance: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Activate_MVCLoadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsActivateMVCLoadResponse {
        #[yaserde(rename = "Ws_Activate_MVCLoadResult", prefix = "tns", default)]
        pub ws_activate_mvc_load_result: ActivateAndMVCLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ActivateAndMVCLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ActivateAndMVCLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "MVCToken", prefix = "tns", default)]
        pub mvc_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Renew_Card",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsRenewCard {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "RenewOptions", prefix = "tns", default)]
        pub renew_options: i32,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "NewProductID", prefix = "tns", default)]
        pub new_product_id: Option<String>,
        #[yaserde(rename = "SendSMS", prefix = "tns", default)]
        pub send_sms: i32,
        #[yaserde(rename = "MailOrSMS", prefix = "tns", default)]
        pub mail_or_sms: Option<String>,
        #[yaserde(rename = "CreateImage", prefix = "tns", default)]
        pub create_image: i32,
        #[yaserde(rename = "FeeWaiver", prefix = "tns", default)]
        pub fee_waiver: Option<String>,
        #[yaserde(rename = "ProductRef", prefix = "tns", default)]
        pub product_ref: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Renew_CardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsRenewCardResponse {
        #[yaserde(rename = "Ws_Renew_CardResult", prefix = "tns", default)]
        pub ws_renew_card_result: Renewal,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Renewal",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Renewal {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "Image", prefix = "tns", default)]
        pub image: Option<String>,
        #[yaserde(rename = "EWALLET", prefix = "tns", default)]
        pub ewallet: Option<ArrayOfEWALLET>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfEWALLET",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfEWALLET {
        #[yaserde(rename = "EWALLET", prefix = "tns", default)]
        pub ewallet: Vec<Ewallet>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCARD",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCARD {
        #[yaserde(rename = "CARD", prefix = "tns", default)]
        pub card: Vec<Card>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CARD",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Card {
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "CRDCURRCODE", prefix = "tns", default)]
        pub crdcurrcode: Option<String>,
        #[yaserde(rename = "CRDPRODUCT", prefix = "tns", default)]
        pub crdproduct: Option<String>,
        #[yaserde(rename = "CUSTCODE", prefix = "tns", default)]
        pub custcode: Option<String>,
        #[yaserde(rename = "PRIMARY", prefix = "tns", default)]
        pub primary: Option<String>,
        #[yaserde(rename = "PROGRAMID", prefix = "tns", default)]
        pub programid: Option<String>,
        #[yaserde(rename = "STATCODE", prefix = "tns", default)]
        pub statcode: Option<String>,
        #[yaserde(rename = "EXPDATE", prefix = "tns", default)]
        pub expdate: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfACCOUNT",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfACCOUNT {
        #[yaserde(rename = "ACCOUNT", prefix = "tns", default)]
        pub account: Vec<Account>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ACCOUNT",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Account {
        #[yaserde(rename = "ACCNO", prefix = "tns", default)]
        pub accno: Option<String>,
        #[yaserde(rename = "ACCTYPE", prefix = "tns", default)]
        pub acctype: Option<String>,
        #[yaserde(rename = "CURRCODE", prefix = "tns", default)]
        pub currcode: Option<String>,
        #[yaserde(rename = "FINAMT", prefix = "tns", default)]
        pub finamt: Option<String>,
        #[yaserde(rename = "BLKAMT", prefix = "tns", default)]
        pub blkamt: Option<String>,
        #[yaserde(rename = "AMTAVL", prefix = "tns", default)]
        pub amtavl: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_ResetAccumulator",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsResetAccumulator {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "AccumulatorType", prefix = "tns", default)]
        pub accumulator_type: i32,
        #[yaserde(rename = "DPAN", prefix = "tns", default)]
        pub dpan: Option<String>,
        #[yaserde(rename = "PaymentTokenId", prefix = "tns", default)]
        pub payment_token_id: Option<String>,
        #[yaserde(rename = "ClearAllDPANs", prefix = "tns", default)]
        pub clear_all_dpa_ns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_ResetAccumulatorResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsResetAccumulatorResponse {
        #[yaserde(rename = "Ws_ResetAccumulatorResult", prefix = "tns", default)]
        pub ws_reset_accumulator_result: Accumulator,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Accumulator",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Accumulator {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Enrol",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsEnrol {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "vClientID", prefix = "tns", default)]
        pub v_client_id: Option<String>,
        #[yaserde(rename = "vCustomerID", prefix = "tns", default)]
        pub v_customer_id: Option<String>,
        #[yaserde(rename = "PAR", prefix = "tns", default)]
        pub par: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_EnrolResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsEnrolResponse {
        #[yaserde(rename = "Ws_EnrolResult", prefix = "tns", default)]
        pub ws_enrol_result: EnrolCardResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "EnrolCardResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnrolCardResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "vCardID", prefix = "tns", default)]
        pub v_card_id: Option<String>,
        #[yaserde(rename = "PAR", prefix = "tns", default)]
        pub par: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Activate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardActivate {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "ActMethod", prefix = "tns", default)]
        pub act_method: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "SMSBalance", prefix = "tns", default)]
        pub sms_balance: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_ActivateResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardActivateResponse {
        #[yaserde(rename = "Ws_GiftCard_ActivateResult", prefix = "tns", default)]
        pub ws_gift_card_activate_result: Activate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Load",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_LoadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardLoadResponse {
        #[yaserde(rename = "Ws_GiftCard_LoadResult", prefix = "tns", default)]
        pub ws_gift_card_load_result: LoadCard,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_UnLoad",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUnLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "AmtUnLoad", prefix = "tns", default)]
        pub amt_un_load: f64,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_UnLoadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUnLoadResponse {
        #[yaserde(rename = "Ws_GiftCard_UnLoadResult", prefix = "tns", default)]
        pub ws_gift_card_un_load_result: UnLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_StatusChange",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardStatusChange {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "NewStatCode", prefix = "tns", default)]
        pub new_stat_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_StatusChangeResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardStatusChangeResponse {
        #[yaserde(rename = "Ws_GiftCard_StatusChangeResult", prefix = "tns", default)]
        pub ws_gift_card_status_change_result: StatusChange,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "AccNo", prefix = "tns", default)]
        pub acc_no: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "SecId", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_EnquiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardEnquiryResponse {
        #[yaserde(rename = "Ws_GiftCard_EnquiryResult", prefix = "tns", default)]
        pub ws_gift_card_enquiry_result: Card,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_BalanceTransfer",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBalanceTransfer {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "NewPAN", prefix = "tns", default)]
        pub new_pan: Option<String>,
        #[yaserde(rename = "NewToken", prefix = "tns", default)]
        pub new_token: Option<String>,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "CurrCode", prefix = "tns", default)]
        pub curr_code: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_BalanceTransferResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBalanceTransferResponse {
        #[yaserde(rename = "Ws_GiftCard_BalanceTransferResult", prefix = "tns", default)]
        pub ws_gift_card_balance_transfer_result: BalanceTransfer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Balance_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBalanceEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Balance_EnquiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBalanceEnquiryResponse {
        #[yaserde(rename = "Ws_GiftCard_Balance_EnquiryResult", prefix = "tns", default)]
        pub ws_gift_card_balance_enquiry_result: BalanceEnquire,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Card_Statement",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardCardStatement {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "TxnFilter", prefix = "tns", default)]
        pub txn_filter: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", prefix = "tns", default)]
        pub num_txn: i32,
        #[yaserde(rename = "DataSrc", prefix = "tns", default)]
        pub data_src: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Card_StatementResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardCardStatementResponse {
        #[yaserde(rename = "Ws_GiftCard_Card_StatementResult", prefix = "tns", default)]
        pub ws_gift_card_card_statement_result: CardStatement,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardStatement",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardStatement {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "StartBal", prefix = "tns", default)]
        pub start_bal: f64,
        #[yaserde(rename = "EndBal", prefix = "tns", default)]
        pub end_bal: f64,
        #[yaserde(rename = "TxnFilter", prefix = "tns", default)]
        pub txn_filter: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", prefix = "tns", default)]
        pub num_txn: i32,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "CurBill", prefix = "tns", default)]
        pub cur_bill: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Transactions", prefix = "tns", default)]
        pub transactions: Option<ArrayOfTransaction>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfTransaction",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfTransaction {
        #[yaserde(rename = "Transaction", prefix = "tns", default)]
        pub transaction: Vec<Transaction>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Transaction",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Transaction {
        #[yaserde(rename = "TxnDate", prefix = "tns", default)]
        pub txn_date: Option<String>,
        #[yaserde(rename = "PostDate", prefix = "tns", default)]
        pub post_date: Option<String>,
        #[yaserde(rename = "AmtBill", prefix = "tns", default)]
        pub amt_bill: f64,
        #[yaserde(rename = "AmtTxn", prefix = "tns", default)]
        pub amt_txn: f64,
        #[yaserde(rename = "BillConvRate", prefix = "tns", default)]
        pub bill_conv_rate: f64,
        #[yaserde(rename = "DebOrCred", prefix = "tns", default)]
        pub deb_or_cred: i32,
        #[yaserde(rename = "TerminalId", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "RRN", prefix = "tns", default)]
        pub rrn: Option<String>,
        #[yaserde(rename = "CurTxn", prefix = "tns", default)]
        pub cur_txn: Option<String>,
        #[yaserde(rename = "ItemId", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "TransactionType", prefix = "tns", default)]
        pub transaction_type: Option<String>,
        #[yaserde(rename = "StatusCode", prefix = "tns", default)]
        pub status_code: Option<String>,
        #[yaserde(rename = "StatusDesc", prefix = "tns", default)]
        pub status_desc: Option<String>,
        #[yaserde(rename = "TxnTime", prefix = "tns", default)]
        pub txn_time: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "FeeId", prefix = "tns", default)]
        pub fee_id: i64,
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "FixedFee", prefix = "tns", default)]
        pub fixed_fee: f64,
        #[yaserde(rename = "RateFee", prefix = "tns", default)]
        pub rate_fee: f64,
        #[yaserde(rename = "FxPdg", prefix = "tns", default)]
        pub fx_pdg: f64,
        #[yaserde(rename = "MCCPdg", prefix = "tns", default)]
        pub mcc_pdg: f64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Update_Cardholder_Details",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUpdateCardholderDetails {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "accCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "newAccCode", prefix = "tns", default)]
        pub new_acc_code: Option<String>,
        #[yaserde(rename = "crdProduct", prefix = "tns", default)]
        pub crd_product: Option<String>,
        #[yaserde(rename = "lastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "firstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "addr1", prefix = "tns", default)]
        pub addr_1: Option<String>,
        #[yaserde(rename = "addr2", prefix = "tns", default)]
        pub addr_2: Option<String>,
        #[yaserde(rename = "city", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "postcode", prefix = "tns", default)]
        pub postcode: Option<String>,
        #[yaserde(rename = "country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "tel", prefix = "tns", default)]
        pub tel: Option<String>,
        #[yaserde(rename = "Workaddr1", prefix = "tns", default)]
        pub workaddr_1: Option<String>,
        #[yaserde(rename = "Workaddr2", prefix = "tns", default)]
        pub workaddr_2: Option<String>,
        #[yaserde(rename = "Workaddr3", prefix = "tns", default)]
        pub workaddr_3: Option<String>,
        #[yaserde(rename = "Workcity", prefix = "tns", default)]
        pub workcity: Option<String>,
        #[yaserde(rename = "Workpostcode", prefix = "tns", default)]
        pub workpostcode: Option<String>,
        #[yaserde(rename = "Workcounty", prefix = "tns", default)]
        pub workcounty: Option<String>,
        #[yaserde(rename = "Workcountry", prefix = "tns", default)]
        pub workcountry: Option<String>,
        #[yaserde(rename = "Worktel", prefix = "tns", default)]
        pub worktel: Option<String>,
        #[yaserde(rename = "pobox", prefix = "tns", default)]
        pub pobox: Option<String>,
        #[yaserde(rename = "email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "fax", prefix = "tns", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "mobTel", prefix = "tns", default)]
        pub mob_tel: Option<String>,
        #[yaserde(rename = "maritalStatus", prefix = "tns", default)]
        pub marital_status: Option<String>,
        #[yaserde(rename = "sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "embossName", prefix = "tns", default)]
        pub emboss_name: Option<String>,
        #[yaserde(rename = "refuseCheck", prefix = "tns", default)]
        pub refuse_check: Option<String>,
        #[yaserde(rename = "mailShots", prefix = "tns", default)]
        pub mail_shots: Option<String>,
        #[yaserde(rename = "discret", prefix = "tns", default)]
        pub discret: Option<String>,
        #[yaserde(rename = "userdata", prefix = "tns", default)]
        pub userdata: Option<String>,
        #[yaserde(rename = "userdata1", prefix = "tns", default)]
        pub userdata_1: Option<String>,
        #[yaserde(rename = "userdata2", prefix = "tns", default)]
        pub userdata_2: Option<String>,
        #[yaserde(rename = "userdata3", prefix = "tns", default)]
        pub userdata_3: Option<String>,
        #[yaserde(rename = "userdata4", prefix = "tns", default)]
        pub userdata_4: Option<String>,
        #[yaserde(rename = "pin", prefix = "tns", default)]
        pub pin: Option<String>,
        #[yaserde(rename = "imageID", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "brncode", prefix = "tns", default)]
        pub brncode: Option<String>,
        #[yaserde(rename = "renew", prefix = "tns", default)]
        pub renew: Option<String>,
        #[yaserde(rename = "dlvMethod", prefix = "tns", default)]
        pub dlv_method: Option<String>,
        #[yaserde(rename = "denyMCC", prefix = "tns", default)]
        pub deny_mcc: Option<String>,
        #[yaserde(rename = "denySvc", prefix = "tns", default)]
        pub deny_svc: Option<String>,
        #[yaserde(rename = "accType", prefix = "tns", default)]
        pub acc_type: Option<String>,
        #[yaserde(rename = "memo", prefix = "tns", default)]
        pub memo: Option<String>,
        #[yaserde(rename = "memoScope", prefix = "tns", default)]
        pub memo_scope: i32,
        #[yaserde(rename = "memoUser", prefix = "tns", default)]
        pub memo_user: Option<String>,
        #[yaserde(rename = "itemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "dlvTitle", prefix = "tns", default)]
        pub dlv_title: Option<String>,
        #[yaserde(rename = "dlvfirstName", prefix = "tns", default)]
        pub dlvfirst_name: Option<String>,
        #[yaserde(rename = "dlvlastName", prefix = "tns", default)]
        pub dlvlast_name: Option<String>,
        #[yaserde(rename = "dlvaddr1", prefix = "tns", default)]
        pub dlvaddr_1: Option<String>,
        #[yaserde(rename = "dlvaddr2", prefix = "tns", default)]
        pub dlvaddr_2: Option<String>,
        #[yaserde(rename = "dlvaddr3", prefix = "tns", default)]
        pub dlvaddr_3: Option<String>,
        #[yaserde(rename = "dlvcity", prefix = "tns", default)]
        pub dlvcity: Option<String>,
        #[yaserde(rename = "dlvpostcode", prefix = "tns", default)]
        pub dlvpostcode: Option<String>,
        #[yaserde(rename = "dlvcounty", prefix = "tns", default)]
        pub dlvcounty: Option<String>,
        #[yaserde(rename = "dlvcountry", prefix = "tns", default)]
        pub dlvcountry: Option<String>,
        #[yaserde(rename = "dlvtel", prefix = "tns", default)]
        pub dlvtel: Option<String>,
        #[yaserde(rename = "dlvEffDate", prefix = "tns", default)]
        pub dlv_eff_date: Option<String>,
        #[yaserde(rename = "dlvDaysValid", prefix = "tns", default)]
        pub dlv_days_valid: i32,
        #[yaserde(rename = "crdprogram", prefix = "tns", default)]
        pub crdprogram: Option<String>,
        #[yaserde(rename = "crddesign", prefix = "tns", default)]
        pub crddesign: Option<String>,
        #[yaserde(rename = "feeTier", prefix = "tns", default)]
        pub fee_tier: Option<String>,
        #[yaserde(rename = "isoLang", prefix = "tns", default)]
        pub iso_lang: Option<String>,
        #[yaserde(rename = "fundcrdPAN", prefix = "tns", default)]
        pub fundcrd_pan: Option<String>,
        #[yaserde(rename = "fundCrdEffDate", prefix = "tns", default)]
        pub fund_crd_eff_date: Option<String>,
        #[yaserde(rename = "fundCrdExpDate", prefix = "tns", default)]
        pub fund_crd_exp_date: Option<String>,
        #[yaserde(rename = "fundCrdType", prefix = "tns", default)]
        pub fund_crd_type: Option<String>,
        #[yaserde(rename = "fundCrdIssNum", prefix = "tns", default)]
        pub fund_crd_iss_num: i32,
        #[yaserde(rename = "fundCrdCVC", prefix = "tns", default)]
        pub fund_crd_cvc: i32,
        #[yaserde(rename = "svcSrc", prefix = "tns", default)]
        pub svc_src: i32,
        #[yaserde(rename = "svcType", prefix = "tns", default)]
        pub svc_type: i32,
        #[yaserde(rename = "svcStatus", prefix = "tns", default)]
        pub svc_status: i32,
        #[yaserde(rename = "secID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "SmsBalance", prefix = "tns", default)]
        pub sms_balance: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Update_Cardholder_DetailsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUpdateCardholderDetailsResponse {
        #[yaserde(
            rename = "Ws_GiftCard_Update_Cardholder_DetailsResult",
            prefix = "tns",
            default
        )]
        pub ws_gift_card_update_cardholder_details_result: CustomerUpdate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_UnLoad_StatusChange",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUnLoadStatusChange {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "AmtUnLoad", prefix = "tns", default)]
        pub amt_un_load: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "StatCode", prefix = "tns", default)]
        pub stat_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_UnLoad_StatusChangeResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUnLoadStatusChangeResponse {
        #[yaserde(
            rename = "Ws_GiftCard_UnLoad_StatusChangeResult",
            prefix = "tns",
            default
        )]
        pub ws_gift_card_un_load_status_change_result: UnLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Activate_Load",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardActivateLoad {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "ActMethod", prefix = "tns", default)]
        pub act_method: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "ActivateOrNot", prefix = "tns", default)]
        pub activate_or_not: i32,
        #[yaserde(rename = "PANorToken", prefix = "tns", default)]
        pub pa_nor_token: i32,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "SMSBalance", prefix = "tns", default)]
        pub sms_balance: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Activate_LoadResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardActivateLoadResponse {
        #[yaserde(rename = "Ws_GiftCard_Activate_LoadResult", prefix = "tns", default)]
        pub ws_gift_card_activate_load_result: ActivateAndLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_BalanceAdjustment",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBalanceAdjustment {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AmtAdjustment", prefix = "tns", default)]
        pub amt_adjustment: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "DebOrCred", prefix = "tns", default)]
        pub deb_or_cred: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "ForcePost", prefix = "tns", default)]
        pub force_post: bool,
        #[yaserde(rename = "ExtCode", prefix = "tns", default)]
        pub ext_code: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_BalanceAdjustmentResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBalanceAdjustmentResponse {
        #[yaserde(
            rename = "Ws_GiftCard_BalanceAdjustmentResult",
            prefix = "tns",
            default
        )]
        pub ws_gift_card_balance_adjustment_result: BalanceAdjust,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_ExtendExpiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardExtendExpiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_ExtendExpiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardExtendExpiryResponse {
        #[yaserde(rename = "Ws_GiftCard_ExtendExpiryResult", prefix = "tns", default)]
        pub ws_gift_card_extend_expiry_result: ExtendExpiry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Transaction_Void",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardTransactionVoid {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "OrgItemId", prefix = "tns", default)]
        pub org_item_id: i64,
        #[yaserde(rename = "Note", prefix = "tns", default)]
        pub note: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Transaction_VoidResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardTransactionVoidResponse {
        #[yaserde(rename = "Ws_GiftCard_Transaction_VoidResult", prefix = "tns", default)]
        pub ws_gift_card_transaction_void_result: TransactionVoid,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_CardHolder_Details_Enquiry",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardCardHolderDetailsEnquiry {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "secID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "secVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_CardHolder_Details_EnquiryResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardCardHolderDetailsEnquiryResponse {
        #[yaserde(
            rename = "Ws_GiftCard_CardHolder_Details_EnquiryResult",
            prefix = "tns",
            default
        )]
        pub ws_gift_card_card_holder_details_enquiry_result: CardHolderDetails,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Phone_Activation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardPhoneActivation {
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "SecurityCode", prefix = "tns", default)]
        pub security_code: Option<String>,
        #[yaserde(rename = "ActivateIfNot", prefix = "tns", default)]
        pub activate_if_not: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Phone_ActivationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardPhoneActivationResponse {
        #[yaserde(rename = "Ws_GiftCard_Phone_ActivationResult", prefix = "tns", default)]
        pub ws_gift_card_phone_activation_result: PhoneActivate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_BulkCreation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBulkCreation {
        #[yaserde(rename = "doc", prefix = "tns", default)]
        pub doc: Option<Doc>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_BulkCreationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardBulkCreationResponse {
        #[yaserde(rename = "Ws_GiftCard_BulkCreationResult", prefix = "tns", default)]
        pub ws_gift_card_bulk_creation_result: BulkCards,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_WebServiceResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardWebServiceResult {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "Isscode", prefix = "tns", default)]
        pub isscode: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_WebServiceResultResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardWebServiceResultResponse {
        #[yaserde(rename = "Ws_GiftCard_WebServiceResultResult", prefix = "tns", default)]
        pub ws_gift_card_web_service_result_result: WsResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Generic_Fees",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardGenericFees {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ProcCode", prefix = "tns", default)]
        pub proc_code: Option<String>,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "Fee", prefix = "tns", default)]
        pub fee: f64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Generic_FeesResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardGenericFeesResponse {
        #[yaserde(rename = "Ws_GiftCard_Generic_FeesResult", prefix = "tns", default)]
        pub ws_gift_card_generic_fees_result: ApplyFees,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_PinControl",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardPinControl {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "locDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "locTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Func", prefix = "tns", default)]
        pub func: Option<String>,
        #[yaserde(rename = "CurrentPin", prefix = "tns", default)]
        pub current_pin: Option<String>,
        #[yaserde(rename = "NewPin", prefix = "tns", default)]
        pub new_pin: Option<String>,
        #[yaserde(rename = "ConfirmPin", prefix = "tns", default)]
        pub confirm_pin: Option<String>,
        #[yaserde(rename = "KeyRef", prefix = "tns", default)]
        pub key_ref: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SecId", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_PinControlResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardPinControlResponse {
        #[yaserde(rename = "Ws_GiftCard_PinControlResult", prefix = "tns", default)]
        pub ws_gift_card_pin_control_result: Pin,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PIN",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Pin {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "CurrentPin", prefix = "tns", default)]
        pub current_pin: Option<String>,
        #[yaserde(rename = "NewPin", prefix = "tns", default)]
        pub new_pin: Option<String>,
        #[yaserde(rename = "KeyRef", prefix = "tns", default)]
        pub key_ref: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "PINStatus", prefix = "tns", default)]
        pub pin_status: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_UpdateLoadSource",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUpdateLoadSource {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ItemId", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_UpdateLoadSourceResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardUpdateLoadSourceResponse {
        #[yaserde(rename = "Ws_GiftCard_UpdateLoadSourceResult", prefix = "tns", default)]
        pub ws_gift_card_update_load_source_result: UpdateLoadSrc,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Activate_Load_ProductTpye_CP",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardActivateLoadProductTpyeCP {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Addrl1", prefix = "tns", default)]
        pub addrl_1: Option<String>,
        #[yaserde(rename = "Addrl2", prefix = "tns", default)]
        pub addrl_2: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "ActMethod", prefix = "tns", default)]
        pub act_method: Option<String>,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "ActivateOrNot", prefix = "tns", default)]
        pub activate_or_not: i32,
        #[yaserde(rename = "PANorToken", prefix = "tns", default)]
        pub pa_nor_token: i32,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "Adv_Permissions", prefix = "tns", default)]
        pub adv_permissions: Option<String>,
        #[yaserde(rename = "LimitsGroup", prefix = "tns", default)]
        pub limits_group: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", prefix = "tns", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_GiftCard_Activate_Load_ProductTpye_CPResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsGiftCardActivateLoadProductTpyeCPResponse {
        #[yaserde(
            rename = "Ws_GiftCard_Activate_Load_ProductTpye_CPResult",
            prefix = "tns",
            default
        )]
        pub ws_gift_card_activate_load_product_tpye_cp_result: ActivateAndLoad,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_TransactionXML",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardTransactionXML {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "AuthType", prefix = "tns", default)]
        pub auth_type: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "Track2", prefix = "tns", default)]
        pub track_2: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "SecID", prefix = "tns", default)]
        pub sec_id: i32,
        #[yaserde(rename = "SecVal", prefix = "tns", default)]
        pub sec_val: Option<String>,
        #[yaserde(rename = "SecValPos", prefix = "tns", default)]
        pub sec_val_pos: i32,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "NumTxn", prefix = "tns", default)]
        pub num_txn: i32,
        #[yaserde(rename = "DataSrc", prefix = "tns", default)]
        pub data_src: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_TransactionXMLResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardTransactionXMLResponse {
        #[yaserde(rename = "Ws_Card_TransactionXMLResult", prefix = "tns", default)]
        pub ws_card_transaction_xml_result: TransactionXML,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Transactions",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Transactions {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransactionXML",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct TransactionXML {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "CurBill", prefix = "tns", default)]
        pub cur_bill: Option<String>,
        #[yaserde(rename = "AvlBal", prefix = "tns", default)]
        pub avl_bal: f64,
        #[yaserde(rename = "BlkAmt", prefix = "tns", default)]
        pub blk_amt: f64,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Transactions", prefix = "tns", default)]
        pub transactions: Option<Transactions>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Change_Groups",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardChangeGroups {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "LimitsGroup", prefix = "tns", default)]
        pub limits_group: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "SchedFeeGroup", prefix = "tns", default)]
        pub sched_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "LinkageGroup", prefix = "tns", default)]
        pub linkage_group: Option<String>,
        #[yaserde(rename = "AuthCalendarGroup", prefix = "tns", default)]
        pub auth_calendar_group: Option<String>,
        #[yaserde(rename = "FXGroup", prefix = "tns", default)]
        pub fx_group: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", prefix = "tns", default)]
        pub payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Change_GroupsResponse",
        namespace = "http://www.globalprocessing.ae/HyperionWeb"
    )]
    pub struct WsCardChangeGroupsResponse {
        #[yaserde(rename = "Ws_Card_Change_GroupsResult", default)]
        pub ws_card_change_groups_result: ChangeGroup,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ChangeGroup")]
    pub struct ChangeGroup {
        #[yaserde(rename = "WSID", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LocDate", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "PublicToken", default)]
        pub public_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Change_Cardacceptor_List",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardChangeCardacceptorList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PAN", prefix = "tns", default)]
        pub pan: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "BlackList", prefix = "tns", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "WhiteList", prefix = "tns", default)]
        pub white_list: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Card_Change_Cardacceptor_ListResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCardChangeCardacceptorListResponse {
        #[yaserde(
            rename = "Ws_Card_Change_Cardacceptor_ListResult",
            prefix = "tns",
            default
        )]
        pub ws_card_change_cardacceptor_list_result: ChangeCardacceptor,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangeCardacceptor",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ChangeCardacceptor {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Change_Cardacceptor_List",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsChangeCardacceptorList {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardSelector", prefix = "tns", default)]
        pub card_selector: i32,
        #[yaserde(rename = "CardSelectorValue", prefix = "tns", default)]
        pub card_selector_value: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "BlackList", prefix = "tns", default)]
        pub black_list: Option<String>,
        #[yaserde(rename = "WhiteList", prefix = "tns", default)]
        pub white_list: Option<String>,
        #[yaserde(rename = "CurrentBlackList", prefix = "tns", default)]
        pub current_black_list: Option<String>,
        #[yaserde(rename = "CurrentWhiteList", prefix = "tns", default)]
        pub current_white_list: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Change_Cardacceptor_ListResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsChangeCardacceptorListResponse {
        #[yaserde(rename = "Ws_Change_Cardacceptor_ListResult", prefix = "tns", default)]
        pub ws_change_cardacceptor_list_result: ChangeAccepter,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangeAccepter",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ChangeAccepter {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "CardSelector", prefix = "tns", default)]
        pub card_selector: i32,
        #[yaserde(rename = "CardSelectorValue", prefix = "tns", default)]
        pub card_selector_value: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "CardsAccepter", prefix = "tns", default)]
        pub cards_accepter: Option<ArrayOfCardAccepterList>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCardAccepterList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfCardAccepterList {
        #[yaserde(rename = "CardAccepterList", prefix = "tns", default)]
        pub card_accepter_list: Vec<CardAccepterList>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardAccepterList",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardAccepterList {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CurrentBlackList", prefix = "tns", default)]
        pub current_black_list: Option<String>,
        #[yaserde(rename = "CurrentWhiteList", prefix = "tns", default)]
        pub current_white_list: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_AddressMatchChecking",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsAddressMatchChecking {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Surname", prefix = "tns", default)]
        pub surname: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "Address1", prefix = "tns", default)]
        pub address_1: Option<String>,
        #[yaserde(rename = "IssuingCountry", prefix = "tns", default)]
        pub issuing_country: Option<String>,
        #[yaserde(rename = "DocumentTypes", prefix = "tns", default)]
        pub document_types: Option<String>,
        #[yaserde(rename = "DocumentNo", prefix = "tns", default)]
        pub document_no: Option<String>,
        #[yaserde(rename = "DocumentExpirydate", prefix = "tns", default)]
        pub document_expirydate: Option<String>,
        #[yaserde(rename = "MobileNo", prefix = "tns", default)]
        pub mobile_no: Option<String>,
        #[yaserde(rename = "ClientIDValue", prefix = "tns", default)]
        pub client_id_value: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_AddressMatchCheckingResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsAddressMatchCheckingResponse {
        #[yaserde(rename = "Ws_AddressMatchCheckingResult", prefix = "tns", default)]
        pub ws_address_match_checking_result: Kyc,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "KYC",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Kyc {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_LicenseVerification",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsLicenseVerification {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "Isscode", prefix = "tns", default)]
        pub isscode: Option<String>,
        #[yaserde(rename = "DocumentNo", prefix = "tns", default)]
        pub document_no: Option<String>,
        #[yaserde(rename = "SurName", prefix = "tns", default)]
        pub sur_name: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "Sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "IssuingCountry", prefix = "tns", default)]
        pub issuing_country: Option<String>,
        #[yaserde(rename = "ClientIDValue", prefix = "tns", default)]
        pub client_id_value: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_LicenseVerificationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsLicenseVerificationResponse {
        #[yaserde(rename = "Ws_LicenseVerificationResult", prefix = "tns", default)]
        pub ws_license_verification_result: Kyc,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_PassportVerification",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPassportVerification {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PassportNo", prefix = "tns", default)]
        pub passport_no: Option<String>,
        #[yaserde(rename = "CheckDigits", prefix = "tns", default)]
        pub check_digits: Option<String>,
        #[yaserde(rename = "Sex", prefix = "tns", default)]
        pub sex: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "SurName", prefix = "tns", default)]
        pub sur_name: Option<String>,
        #[yaserde(rename = "IssuingCountry", prefix = "tns", default)]
        pub issuing_country: Option<String>,
        #[yaserde(rename = "DocumentExpirydate", prefix = "tns", default)]
        pub document_expirydate: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "ClientIDValue", prefix = "tns", default)]
        pub client_id_value: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_PassportVerificationResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsPassportVerificationResponse {
        #[yaserde(rename = "Ws_PassportVerificationResult", prefix = "tns", default)]
        pub ws_passport_verification_result: Kyc,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Sanctions_PEP_Check",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSanctionsPEPCheck {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Nationality", prefix = "tns", default)]
        pub nationality: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "Postcode", prefix = "tns", default)]
        pub postcode: Option<String>,
        #[yaserde(rename = "Address1", prefix = "tns", default)]
        pub address_1: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Sanctions_PEP_CheckResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSanctionsPEPCheckResponse {
        #[yaserde(rename = "Ws_Sanctions_PEP_CheckResult", prefix = "tns", default)]
        pub ws_sanctions_pep_check_result: SanctionsPEP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Sanctions_PEP",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct SanctionsPEP {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: bool,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Sanctions_PEP_Check_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSanctionsPEPCheckV2 {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "Nationality", prefix = "tns", default)]
        pub nationality: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "Postcode", prefix = "tns", default)]
        pub postcode: Option<String>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "checkLevel", prefix = "tns", default)]
        pub check_level: Option<String>,
        #[yaserde(rename = "MatchItems", prefix = "tns", default)]
        pub match_items: Option<String>,
        #[yaserde(rename = "ProductID", prefix = "tns", default)]
        pub product_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Sanctions_PEP_Check_V2Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsSanctionsPEPCheckV2Response {
        #[yaserde(rename = "Ws_Sanctions_PEP_Check_V2Result", prefix = "tns", default)]
        pub ws_sanctions_pep_check_v2_result: SanctionsPEPV2,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Sanctions_PEP",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListSanctionsPEP {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Sanctions_PEPResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListSanctionsPEPResponse {
        #[yaserde(rename = "Ws_List_Sanctions_PEPResult", prefix = "tns", default)]
        pub ws_list_sanctions_pep_result: ListSanctionPEP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListSanctionPEP",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ListSanctionPEP {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<String>,
        #[yaserde(rename = "Source", prefix = "tns", default)]
        pub source: Option<String>,
        #[yaserde(rename = "ListedOn", prefix = "tns", default)]
        pub listed_on: Option<String>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "DateOfBirths", prefix = "tns", default)]
        pub date_of_births: Option<ArrayOfDateOfBirth>,
        #[yaserde(rename = "PlaceOfBirths", prefix = "tns", default)]
        pub place_of_births: Option<ArrayOfPlaceOfBirth>,
        #[yaserde(rename = "Nationalities", prefix = "tns", default)]
        pub nationalities: Option<ArrayOfNationality>,
        #[yaserde(rename = "Passports", prefix = "tns", default)]
        pub passports: Option<ArrayOfPassport>,
        #[yaserde(rename = "NationalIdentificationNumbers", prefix = "tns", default)]
        pub national_identification_numbers: Option<ArrayOfNationalIdentificationNumber>,
        #[yaserde(rename = "Positions", prefix = "tns", default)]
        pub positions: Option<ArrayOfPosition>,
        #[yaserde(rename = "Addresses", prefix = "tns", default)]
        pub addresses: Option<ArrayOfAddress>,
        #[yaserde(rename = "Aliases", prefix = "tns", default)]
        pub aliases: Option<ArrayOfAka>,
        #[yaserde(rename = "Regimes", prefix = "tns", default)]
        pub regimes: Option<ArrayOfRegime>,
        #[yaserde(rename = "OtherInfomations", prefix = "tns", default)]
        pub other_infomations: Option<ArrayOfOtherInfomation>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfDateOfBirth",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfDateOfBirth {
        #[yaserde(rename = "DateOfBirth", prefix = "tns", default)]
        pub date_of_birth: Vec<DateOfBirth>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DateOfBirth",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct DateOfBirth {
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPlaceOfBirth",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfPlaceOfBirth {
        #[yaserde(rename = "PlaceOfBirth", prefix = "tns", default)]
        pub place_of_birth: Vec<PlaceOfBirth>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PlaceOfBirth",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct PlaceOfBirth {
        #[yaserde(rename = "POB", prefix = "tns", default)]
        pub pob: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfNationality",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfNationality {
        #[yaserde(rename = "Nationality", prefix = "tns", default)]
        pub nationality: Vec<Nationality>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Nationality",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Nationality {
        #[yaserde(rename = "Nation", prefix = "tns", default)]
        pub nation: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPassport",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfPassport {
        #[yaserde(rename = "Passport", prefix = "tns", default)]
        pub passport: Vec<Passport>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Passport",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Passport {
        #[yaserde(rename = "PassportNumber", prefix = "tns", default)]
        pub passport_number: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfNationalIdentificationNumber",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfNationalIdentificationNumber {
        #[yaserde(rename = "NationalIdentificationNumber", prefix = "tns", default)]
        pub national_identification_number: Vec<NationalIdentificationNumber>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "NationalIdentificationNumber",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct NationalIdentificationNumber {
        #[yaserde(rename = "NID", prefix = "tns", default)]
        pub nid: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPosition",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfPosition {
        #[yaserde(rename = "Position", prefix = "tns", default)]
        pub position: Vec<Position>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Position",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Position {
        #[yaserde(rename = "Post", prefix = "tns", default)]
        pub post: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAddress",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfAddress {
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Vec<Address>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Address",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Address {
        #[yaserde(rename = "Address1", prefix = "tns", default)]
        pub address_1: Option<String>,
        #[yaserde(rename = "Address2", prefix = "tns", default)]
        pub address_2: Option<String>,
        #[yaserde(rename = "Address3", prefix = "tns", default)]
        pub address_3: Option<String>,
        #[yaserde(rename = "Address4", prefix = "tns", default)]
        pub address_4: Option<String>,
        #[yaserde(rename = "Address5", prefix = "tns", default)]
        pub address_5: Option<String>,
        #[yaserde(rename = "Address6", prefix = "tns", default)]
        pub address_6: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAka",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfAka {
        #[yaserde(rename = "Aka", prefix = "tns", default)]
        pub aka: Vec<Aka>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Aka",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Aka {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfRegime",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfRegime {
        #[yaserde(rename = "Regime", prefix = "tns", default)]
        pub regime: Vec<Regime>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Regime",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Regime {
        #[yaserde(rename = "Regiment", prefix = "tns", default)]
        pub regiment: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfOtherInfomation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfOtherInfomation {
        #[yaserde(rename = "OtherInfomation", prefix = "tns", default)]
        pub other_infomation: Vec<OtherInfomation>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OtherInfomation",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct OtherInfomation {
        #[yaserde(rename = "OtherInfo", prefix = "tns", default)]
        pub other_info: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Sanctions_PEP_Matches",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListSanctionsPEPMatches {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "ListType", prefix = "tns", default)]
        pub list_type: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "EndDate", prefix = "tns", default)]
        pub end_date: Option<String>,
        #[yaserde(rename = "Flag", prefix = "tns", default)]
        pub flag: Option<String>,
        #[yaserde(rename = "ProductID", prefix = "tns", default)]
        pub product_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_List_Sanctions_PEP_MatchesResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsListSanctionsPEPMatchesResponse {
        #[yaserde(
            rename = "Ws_List_Sanctions_PEP_MatchesResult",
            prefix = "tns",
            default
        )]
        pub ws_list_sanctions_pep_matches_result: ListSanctionPEPMatches,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListSanctionPEPMatches",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ListSanctionPEPMatches {
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "SanctionPEPMatches", prefix = "tns", default)]
        pub sanction_pep_matches: Option<ArrayOfSanctionPEPMatch>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfSanctionPEPMatch",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfSanctionPEPMatch {
        #[yaserde(rename = "SanctionPEPMatch", prefix = "tns", default)]
        pub sanction_pep_match: Vec<SanctionPEPMatch>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SanctionPEPMatch",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct SanctionPEPMatch {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "Firstname", prefix = "tns", default)]
        pub firstname: Option<String>,
        #[yaserde(rename = "Lastname", prefix = "tns", default)]
        pub lastname: Option<String>,
        #[yaserde(rename = "Nationality", prefix = "tns", default)]
        pub nationality: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "ListType", prefix = "tns", default)]
        pub list_type: Option<String>,
        #[yaserde(rename = "RecordID", prefix = "tns", default)]
        pub record_id: Option<String>,
        #[yaserde(rename = "ReportedDate", prefix = "tns", default)]
        pub reported_date: Option<String>,
        #[yaserde(rename = "LoggedDate", prefix = "tns", default)]
        pub logged_date: Option<String>,
        #[yaserde(rename = "MatchFlag", prefix = "tns", default)]
        pub match_flag: bool,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "Flag", prefix = "tns", default)]
        pub flag: i32,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
        #[yaserde(rename = "Id", prefix = "tns", default)]
        pub id: i32,
        #[yaserde(rename = "CardHolder", prefix = "tns", default)]
        pub card_holder: CardHolder,
        #[yaserde(rename = "CardPurchaser", prefix = "tns", default)]
        pub card_purchaser: CardHolder,
        #[yaserde(rename = "MatchItems", prefix = "tns", default)]
        pub match_items: i32,
        #[yaserde(rename = "ProductID", prefix = "tns", default)]
        pub product_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CardHolder",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CardHolder {
        #[yaserde(rename = "Firstname", prefix = "tns", default)]
        pub firstname: Option<String>,
        #[yaserde(rename = "Lastname", prefix = "tns", default)]
        pub lastname: Option<String>,
        #[yaserde(rename = "DOB", prefix = "tns", default)]
        pub dob: Option<String>,
        #[yaserde(rename = "PostCode", prefix = "tns", default)]
        pub post_code: Option<String>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "Nationality", prefix = "tns", default)]
        pub nationality: Option<String>,
        #[yaserde(rename = "MatchItems", prefix = "tns", default)]
        pub match_items: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Update_Sanctions_PEP_Matches",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateSanctionsPEPMatches {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Id", prefix = "tns", default)]
        pub id: i32,
        #[yaserde(rename = "Flag", prefix = "tns", default)]
        pub flag: i32,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Update_Sanctions_PEP_MatchesResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsUpdateSanctionsPEPMatchesResponse {
        #[yaserde(
            rename = "Ws_Update_Sanctions_PEP_MatchesResult",
            prefix = "tns",
            default
        )]
        pub ws_update_sanctions_pep_matches_result: UpdateSanctionPEP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateSanctionPEP",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct UpdateSanctionPEP {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "Id", prefix = "tns", default)]
        pub id: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CreateCard_V2",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCreateCardV2 {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: CreateBankingEnabledCardRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateBankingEnabledCardRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CreateBankingEnabledCardRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "clientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "Address1", prefix = "tns", default)]
        pub address_1: Option<String>,
        #[yaserde(rename = "Address2", prefix = "tns", default)]
        pub address_2: Option<String>,
        #[yaserde(rename = "Address3", prefix = "tns", default)]
        pub address_3: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "Postcode", prefix = "tns", default)]
        pub postcode: Option<String>,
        #[yaserde(rename = "ISOCountryCode", prefix = "tns", default)]
        pub iso_country_code: Option<String>,
        #[yaserde(rename = "Mobile", prefix = "tns", default)]
        pub mobile: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "CardDesign", prefix = "tns", default)]
        pub card_design: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "DateOfBirth", prefix = "tns", default)]
        pub date_of_birth: Option<String>,
        #[yaserde(rename = "AccCode", prefix = "tns", default)]
        pub acc_code: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "TerminalID", prefix = "tns", default)]
        pub terminal_id: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "CurCode", prefix = "tns", default)]
        pub cur_code: Option<String>,
        #[yaserde(rename = "Reason", prefix = "tns", default)]
        pub reason: Option<String>,
        #[yaserde(rename = "ItemSrc", prefix = "tns", default)]
        pub item_src: i32,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "LoadFundsType", prefix = "tns", default)]
        pub load_funds_type: Option<String>,
        #[yaserde(rename = "LoadSrc", prefix = "tns", default)]
        pub load_src: Option<String>,
        #[yaserde(rename = "LoadFee", prefix = "tns", default)]
        pub load_fee: f64,
        #[yaserde(rename = "LoadedBy", prefix = "tns", default)]
        pub loaded_by: Option<String>,
        #[yaserde(rename = "CreateImage", prefix = "tns", default)]
        pub create_image: i32,
        #[yaserde(rename = "CreateType", prefix = "tns", default)]
        pub create_type: Option<String>,
        #[yaserde(rename = "CustAccount", prefix = "tns", default)]
        pub cust_account: Option<String>,
        #[yaserde(rename = "ActivateNow", prefix = "tns", default)]
        pub activate_now: i32,
        #[yaserde(rename = "SourceDesc", prefix = "tns", default)]
        pub source_desc: Option<String>,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "ExpDate", prefix = "tns", default)]
        pub exp_date: Option<String>,
        #[yaserde(rename = "CVV", prefix = "tns", default)]
        pub cvv: Option<String>,
        #[yaserde(rename = "CardName", prefix = "tns", default)]
        pub card_name: Option<String>,
        #[yaserde(rename = "MaskedPAN", prefix = "tns", default)]
        pub masked_pan: Option<String>,
        #[yaserde(rename = "LimitsGroup", prefix = "tns", default)]
        pub limits_group: Option<String>,
        #[yaserde(rename = "MCCGroup", prefix = "tns", default)]
        pub mcc_group: Option<String>,
        #[yaserde(rename = "PERMSGroup", prefix = "tns", default)]
        pub perms_group: Option<String>,
        #[yaserde(rename = "FeeGroup", prefix = "tns", default)]
        pub fee_group: Option<String>,
        #[yaserde(rename = "ScheduledFeeGroup", prefix = "tns", default)]
        pub scheduled_fee_group: Option<String>,
        #[yaserde(rename = "WSFeeGroup", prefix = "tns", default)]
        pub ws_fee_group: Option<String>,
        #[yaserde(rename = "ProductRef", prefix = "tns", default)]
        pub product_ref: Option<String>,
        #[yaserde(rename = "CarrierType", prefix = "tns", default)]
        pub carrier_type: Option<String>,
        #[yaserde(rename = "Fulfil1", prefix = "tns", default)]
        pub fulfil_1: Option<String>,
        #[yaserde(rename = "Fulfil2", prefix = "tns", default)]
        pub fulfil_2: Option<String>,
        #[yaserde(rename = "DelvMeth", prefix = "tns", default)]
        pub delv_meth: Option<String>,
        #[yaserde(rename = "ThermalLine1", prefix = "tns", default)]
        pub thermal_line_1: Option<String>,
        #[yaserde(rename = "ThermalLine2", prefix = "tns", default)]
        pub thermal_line_2: Option<String>,
        #[yaserde(rename = "Lang", prefix = "tns", default)]
        pub lang: Option<String>,
        #[yaserde(rename = "EmbossLine4", prefix = "tns", default)]
        pub emboss_line_4: Option<String>,
        #[yaserde(rename = "ImageID", prefix = "tns", default)]
        pub image_id: Option<String>,
        #[yaserde(rename = "LogoFrontID", prefix = "tns", default)]
        pub logo_front_id: Option<String>,
        #[yaserde(rename = "LogoBackID", prefix = "tns", default)]
        pub logo_back_id: Option<String>,
        #[yaserde(rename = "Replacement", prefix = "tns", default)]
        pub replacement: bool,
        #[yaserde(rename = "PrimaryToken", prefix = "tns", default)]
        pub primary_token: Option<String>,
        #[yaserde(rename = "DelvAddrL1", prefix = "tns", default)]
        pub delv_addr_l1: Option<String>,
        #[yaserde(rename = "DelvAddrL2", prefix = "tns", default)]
        pub delv_addr_l2: Option<String>,
        #[yaserde(rename = "DelvAddrL3", prefix = "tns", default)]
        pub delv_addr_l3: Option<String>,
        #[yaserde(rename = "DelvCity", prefix = "tns", default)]
        pub delv_city: Option<String>,
        #[yaserde(rename = "DelvPostcode", prefix = "tns", default)]
        pub delv_postcode: Option<String>,
        #[yaserde(rename = "DelvCounty", prefix = "tns", default)]
        pub delv_county: Option<String>,
        #[yaserde(rename = "DelvCountry", prefix = "tns", default)]
        pub delv_country: Option<String>,
        #[yaserde(rename = "DelvCode", prefix = "tns", default)]
        pub delv_code: Option<String>,
        #[yaserde(rename = "SMSRequired", prefix = "tns", default)]
        pub sms_required: Option<String>,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: Option<String>,
        #[yaserde(rename = "CardManufacturer", prefix = "tns", default)]
        pub card_manufacturer: Option<String>,
        #[yaserde(rename = "CoBrand", prefix = "tns", default)]
        pub co_brand: Option<String>,
        #[yaserde(rename = "ExternalAuth", prefix = "tns", default)]
        pub external_auth: Option<String>,
        #[yaserde(rename = "LinkageGroup", prefix = "tns", default)]
        pub linkage_group: Option<String>,
        #[yaserde(rename = "VanityName", prefix = "tns", default)]
        pub vanity_name: Option<String>,
        #[yaserde(rename = "PBlock", prefix = "tns", default)]
        pub p_block: Option<String>,
        #[yaserde(rename = "PINMailer", prefix = "tns", default)]
        pub pin_mailer: Option<String>,
        #[yaserde(rename = "FXGroup", prefix = "tns", default)]
        pub fx_group: Option<String>,
        #[yaserde(rename = "Email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "MailOrSMS", prefix = "tns", default)]
        pub mail_or_sms: Option<String>,
        #[yaserde(rename = "Quantity", prefix = "tns", default)]
        pub quantity: Option<String>,
        #[yaserde(rename = "AuthCalendarGroup", prefix = "tns", default)]
        pub auth_calendar_group: Option<String>,
        #[yaserde(rename = "LoadToken", prefix = "tns", default)]
        pub load_token: Option<String>,
        #[yaserde(rename = "RequiredBankingFeatures", prefix = "tns", default)]
        pub required_banking_features: BankingFeatures,
        #[yaserde(rename = "IsActive", prefix = "tns", default)]
        pub is_active: bool,
        #[yaserde(rename = "CustomerID", prefix = "tns", default)]
        pub customer_id: Option<String>,
        #[yaserde(rename = "PaymentTokenUsageGroup", prefix = "tns", default)]
        pub payment_token_usage_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BankingFeatures",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BankingFeatures {
        #[yaserde(rename = "ExistingToken", prefix = "tns", default)]
        pub existing_token: Option<String>,
        #[yaserde(rename = "BankingInEnabled", prefix = "tns", default)]
        pub banking_in_enabled: bool,
        #[yaserde(rename = "BankingOutEnabled", prefix = "tns", default)]
        pub banking_out_enabled: bool,
        #[yaserde(rename = "DirectDebitInEnabled", prefix = "tns", default)]
        pub direct_debit_in_enabled: bool,
        #[yaserde(rename = "DirectDebitOutEnabled", prefix = "tns", default)]
        pub direct_debit_out_enabled: bool,
        #[yaserde(rename = "CardEnabled", prefix = "tns", default)]
        pub card_enabled: bool,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<String>,
        #[yaserde(rename = "CompanyName", prefix = "tns", default)]
        pub company_name: Option<String>,
        #[yaserde(rename = "AccountName", prefix = "tns", default)]
        pub account_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_CreateCard_V2Response",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsCreateCardV2Response {
        #[yaserde(rename = "Ws_CreateCard_V2Result", prefix = "tns", default)]
        pub ws_create_card_v2_result: CreateBankingEnabledCardResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateBankingEnabledCardResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct CreateBankingEnabledCardResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "TxnCode", prefix = "tns", default)]
        pub txn_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: Option<String>,
        #[yaserde(rename = "ExternalRef", prefix = "tns", default)]
        pub external_ref: Option<String>,
        #[yaserde(rename = "LocDate", prefix = "tns", default)]
        pub loc_date: Option<String>,
        #[yaserde(rename = "LocTime", prefix = "tns", default)]
        pub loc_time: Option<String>,
        #[yaserde(rename = "ItemID", prefix = "tns", default)]
        pub item_id: i64,
        #[yaserde(rename = "ClientCode", prefix = "tns", default)]
        pub client_code: Option<String>,
        #[yaserde(rename = "SysDate", prefix = "tns", default)]
        pub sys_date: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "LoadValue", prefix = "tns", default)]
        pub load_value: f64,
        #[yaserde(rename = "IsLive", prefix = "tns", default)]
        pub is_live: bool,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: Option<String>,
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "AccountNumber", prefix = "tns", default)]
        pub account_number: Option<String>,
        #[yaserde(rename = "AccountName", prefix = "tns", default)]
        pub account_name: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "Messages", prefix = "tns", default)]
        pub messages: Option<ArrayOfBankingError>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_ReturnBankDetailsFromToken",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingReturnBankDetailsFromToken {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: ReturnBankDetailsFromTokenRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ReturnBankDetailsFromTokenRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ReturnBankDetailsFromTokenRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_ReturnBankDetailsFromTokenResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingReturnBankDetailsFromTokenResponse {
        #[yaserde(
            rename = "Ws_Banking_ReturnBankDetailsFromTokenResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_return_bank_details_from_token_result: ReturnBankDetailsFromTokenResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ReturnBankDetailsFromTokenResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ReturnBankDetailsFromTokenResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i64,
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "AccountNumber", prefix = "tns", default)]
        pub account_number: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_ChangeAccountBankingFeaturesStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingChangeAccountBankingFeaturesStatus {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: ChangeAccountBankingFeaturesStatusRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangeAccountBankingFeaturesStatusRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ChangeAccountBankingFeaturesStatusRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "UpdateSubAccountsToSame", prefix = "tns", default)]
        pub update_sub_accounts_to_same: bool,
        #[yaserde(rename = "BankingFeatures", prefix = "tns", default)]
        pub banking_features: BankingFeaturesType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BankingFeaturesType",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BankingFeaturesType {
        #[yaserde(rename = "BankingIn", prefix = "tns", default)]
        pub banking_in: EnumBankingIn,
        #[yaserde(rename = "BankingOut", prefix = "tns", default)]
        pub banking_out: EnumBankingOut,
        #[yaserde(rename = "DirectDebitIn", prefix = "tns", default)]
        pub direct_debit_in: EnumDirectDebitIn,
        #[yaserde(rename = "DirectDebitOut", prefix = "tns", default)]
        pub direct_debit_out: EnumDirectDebitOut,
        #[yaserde(rename = "AccountName", prefix = "tns", default)]
        pub account_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_BankingIn",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumBankingIn {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_BankingOut",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumBankingOut {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitIn",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitIn {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_DirectDebitOut",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumDirectDebitOut {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_ChangeAccountBankingFeaturesStatusResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingChangeAccountBankingFeaturesStatusResponse {
        #[yaserde(
            rename = "Ws_Banking_ChangeAccountBankingFeaturesStatusResult",
            prefix = "tns",
            default
        )]
        pub ws_banking_change_account_banking_features_status_result:
            ChangeAccountBankingFeaturesStatusResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangeAccountBankingFeaturesStatusResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ChangeAccountBankingFeaturesStatusResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: i64,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "Response", prefix = "tns", default)]
        pub response: EnumChangeAccountStatus,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "Accounts", prefix = "tns", default)]
        pub accounts: Option<ArrayOfChangedBankingFeatures>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_ChangeAccountStatus",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumChangeAccountStatus {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfChangedBankingFeatures",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ArrayOfChangedBankingFeatures {
        #[yaserde(rename = "ChangedBankingFeatures", prefix = "tns", default)]
        pub changed_banking_features: Vec<ChangedBankingFeatures>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangedBankingFeatures",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct ChangedBankingFeatures {
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "BankingFeatures", prefix = "tns", default)]
        pub banking_features: BankingFeaturesReturnType,
        #[yaserde(rename = "UpdateSuccess", prefix = "tns", default)]
        pub update_success: EnumChangedBankingFeaturesUpdate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "BankingFeaturesReturnType",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct BankingFeaturesReturnType {
        #[yaserde(rename = "BankingIn", prefix = "tns", default)]
        pub banking_in: EnumBankingInEnabled,
        #[yaserde(rename = "BankingOut", prefix = "tns", default)]
        pub banking_out: EnumBankingOutEnabled,
        #[yaserde(rename = "DirectDebitIn", prefix = "tns", default)]
        pub direct_debit_in: EnumDirectDebitInEnabled,
        #[yaserde(rename = "DirectDebitOut", prefix = "tns", default)]
        pub direct_debit_out: EnumDirectDebitOutEnabled,
        #[yaserde(rename = "CardEnabled", prefix = "tns", default)]
        pub card_enabled: EnumCardEnabled,
        #[yaserde(rename = "AccountName", prefix = "tns", default)]
        pub account_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_ChangedBankingFeaturesUpdate",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumChangedBankingFeaturesUpdate {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_TransferFunds",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingTransferFunds {
        #[yaserde(rename = "request", prefix = "tns", default)]
        pub request: TransferFundsRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferFundsRequest",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct TransferFundsRequest {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "IssCode", prefix = "tns", default)]
        pub iss_code: Option<String>,
        #[yaserde(rename = "PublicToken", prefix = "tns", default)]
        pub public_token: i32,
        #[yaserde(rename = "Beneficiary", prefix = "tns", default)]
        pub beneficiary: Beneficiary,
        #[yaserde(rename = "Reference", prefix = "tns", default)]
        pub reference: Option<String>,
        #[yaserde(rename = "AmountOfPayment", prefix = "tns", default)]
        pub amount_of_payment: f64,
        #[yaserde(rename = "AuthorisedBy", prefix = "tns", default)]
        pub authorised_by: Option<String>,
        #[yaserde(rename = "Currency", prefix = "tns", default)]
        pub currency: Option<String>,
        #[yaserde(rename = "PaymentMethod", prefix = "tns", default)]
        pub payment_method: EnumPaymentMethod,
        #[yaserde(rename = "Direction", prefix = "tns", default)]
        pub direction: EnumPaymentDirection,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Beneficiary",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct Beneficiary {
        #[yaserde(rename = "AccountNumber", prefix = "tns", default)]
        pub account_number: Option<String>,
        #[yaserde(rename = "SortCode", prefix = "tns", default)]
        pub sort_code: Option<String>,
        #[yaserde(rename = "AccountName", prefix = "tns", default)]
        pub account_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_PaymentMethod",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumPaymentMethod {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_PaymentDirection",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumPaymentDirection {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Ws_Banking_TransferFundsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct WsBankingTransferFundsResponse {
        #[yaserde(rename = "Ws_Banking_TransferFundsResult", prefix = "tns", default)]
        pub ws_banking_transfer_funds_result: TransferFundsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferFundsResponse",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct TransferFundsResponse {
        #[yaserde(rename = "WSID", prefix = "tns", default)]
        pub wsid: Option<String>,
        #[yaserde(rename = "Response", prefix = "tns", default)]
        pub response: EnumTransferFundsResult,
        #[yaserde(rename = "TransactionID", prefix = "tns", default)]
        pub transaction_id: i64,
        #[yaserde(rename = "ErrorText", prefix = "tns", default)]
        pub error_text: Option<String>,
        #[yaserde(rename = "ActionCode", prefix = "tns", default)]
        pub action_code: Option<String>,
        #[yaserde(rename = "RecipientActionCode", prefix = "tns", default)]
        pub recipient_action_code: Option<String>,
        #[yaserde(rename = "RecipientTranactionID", prefix = "tns", default)]
        pub recipient_tranaction_id: Option<String>,
        #[yaserde(rename = "BankTransactionId", prefix = "tns", default)]
        pub bank_transaction_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ENUM_TransferFundsResult",
        namespace = "tns: http://www.globalprocessing.ae/HyperionWeb",
        prefix = "tns"
    )]
    pub struct EnumTransferFundsResult {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "guid",
        namespace = "tns: http://microsoft.com/wsdl/types/",
        prefix = "tns"
    )]
    pub struct Guid {
        #[yaserde(text, default)]
        pub body: String,
    }
}

pub mod ports {
    use super::*;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type WsBankingUpdateBankingEnabledCardSoapIn =
        messages::WsBankingUpdateBankingEnabledCardSoapIn;

    pub type WsBankingUpdateBankingEnabledCardSoapOut =
        messages::WsBankingUpdateBankingEnabledCardSoapOut;

    pub type WsBankingStatusQueryBankingEnabledCardSoapIn =
        messages::WsBankingStatusQueryBankingEnabledCardSoapIn;

    pub type WsBankingStatusQueryBankingEnabledCardSoapOut =
        messages::WsBankingStatusQueryBankingEnabledCardSoapOut;

    pub type WsBankingCreateCustomerSoapIn = messages::WsBankingCreateCustomerSoapIn;

    pub type WsBankingCreateCustomerSoapOut = messages::WsBankingCreateCustomerSoapOut;

    pub type WsBankingRegisterNotificationSoapIn = messages::WsBankingRegisterNotificationSoapIn;

    pub type WsBankingRegisterNotificationSoapOut = messages::WsBankingRegisterNotificationSoapOut;

    pub type WsBankingAccountModulusCheckSoapIn = messages::WsBankingAccountModulusCheckSoapIn;

    pub type WsBankingAccountModulusCheckSoapOut = messages::WsBankingAccountModulusCheckSoapOut;

    pub type WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn =
        messages::WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn;

    pub type WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut =
        messages::WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut;

    pub type WsBankingCancelDirectDebitBankingEnabledCardSoapIn =
        messages::WsBankingCancelDirectDebitBankingEnabledCardSoapIn;

    pub type WsBankingCancelDirectDebitBankingEnabledCardSoapOut =
        messages::WsBankingCancelDirectDebitBankingEnabledCardSoapOut;

    pub type WsBankingGetPendingDirectDebitsSoapIn =
        messages::WsBankingGetPendingDirectDebitsSoapIn;

    pub type WsBankingGetPendingDirectDebitsSoapOut =
        messages::WsBankingGetPendingDirectDebitsSoapOut;

    pub type WsBankingCardStatementV2SoapIn = messages::WsBankingCardStatementV2SoapIn;

    pub type WsBankingCardStatementV2SoapOut = messages::WsBankingCardStatementV2SoapOut;

    pub type WsInsert3DSecureDetailsSoapIn = messages::WsInsert3DSecureDetailsSoapIn;

    pub type WsInsert3DSecureDetailsSoapOut = messages::WsInsert3DSecureDetailsSoapOut;

    pub type WsUpdate3DSecureDetailsSoapIn = messages::WsUpdate3DSecureDetailsSoapIn;

    pub type WsUpdate3DSecureDetailsSoapOut = messages::WsUpdate3DSecureDetailsSoapOut;

    pub type WsUpdateLastModifiedTypeSoapIn = messages::WsUpdateLastModifiedTypeSoapIn;

    pub type WsUpdateLastModifiedTypeSoapOut = messages::WsUpdateLastModifiedTypeSoapOut;

    pub type WsDelete3DSecureDetailsSoapIn = messages::WsDelete3DSecureDetailsSoapIn;

    pub type WsDelete3DSecureDetailsSoapOut = messages::WsDelete3DSecureDetailsSoapOut;

    pub type WsQuery3DSecureDetailsSoapIn = messages::WsQuery3DSecureDetailsSoapIn;

    pub type WsQuery3DSecureDetailsSoapOut = messages::WsQuery3DSecureDetailsSoapOut;

    pub type WsGPSLockUnlockSoapIn = messages::WsGPSLockUnlockSoapIn;

    pub type WsGPSLockUnlockSoapOut = messages::WsGPSLockUnlockSoapOut;

    pub type WsVerificationRequestSoapIn = messages::WsVerificationRequestSoapIn;

    pub type WsVerificationRequestSoapOut = messages::WsVerificationRequestSoapOut;

    pub type WsAddUpDelCredentialsSoapIn = messages::WsAddUpDelCredentialsSoapIn;

    pub type WsAddUpDelCredentialsSoapOut = messages::WsAddUpDelCredentialsSoapOut;

    pub type Ws3DSAddUpDelDetailsSoapIn = messages::Ws3DSAddUpDelDetailsSoapIn;

    pub type Ws3DSAddUpDelDetailsSoapOut = messages::Ws3DSAddUpDelDetailsSoapOut;

    pub type WsBalanceUpdateSoapIn = messages::WsBalanceUpdateSoapIn;

    pub type WsBalanceUpdateSoapOut = messages::WsBalanceUpdateSoapOut;

    pub type WsPaymentTokenGetSoapIn = messages::WsPaymentTokenGetSoapIn;

    pub type WsPaymentTokenGetSoapOut = messages::WsPaymentTokenGetSoapOut;

    pub type WsPaymentTokenStatusChangeSoapIn = messages::WsPaymentTokenStatusChangeSoapIn;

    pub type WsPaymentTokenStatusChangeSoapOut = messages::WsPaymentTokenStatusChangeSoapOut;

    pub type WsActivateSoapIn = messages::WsActivateSoapIn;

    pub type WsActivateSoapOut = messages::WsActivateSoapOut;

    pub type WsLoadSoapIn = messages::WsLoadSoapIn;

    pub type WsLoadSoapOut = messages::WsLoadSoapOut;

    pub type WsUnLoadSoapIn = messages::WsUnLoadSoapIn;

    pub type WsUnLoadSoapOut = messages::WsUnLoadSoapOut;

    pub type WsStatusChangeSoapIn = messages::WsStatusChangeSoapIn;

    pub type WsStatusChangeSoapOut = messages::WsStatusChangeSoapOut;

    pub type WsEnquirySoapIn = messages::WsEnquirySoapIn;

    pub type WsEnquirySoapOut = messages::WsEnquirySoapOut;

    pub type WsBalanceTransferSoapIn = messages::WsBalanceTransferSoapIn;

    pub type WsBalanceTransferSoapOut = messages::WsBalanceTransferSoapOut;

    pub type WsBalanceEnquirySoapIn = messages::WsBalanceEnquirySoapIn;

    pub type WsBalanceEnquirySoapOut = messages::WsBalanceEnquirySoapOut;

    pub type WsBalanceEnquiryRepSoapIn = messages::WsBalanceEnquiryRepSoapIn;

    pub type WsBalanceEnquiryRepSoapOut = messages::WsBalanceEnquiryRepSoapOut;

    pub type WsBalanceEnquiryV2SoapIn = messages::WsBalanceEnquiryV2SoapIn;

    pub type WsBalanceEnquiryV2SoapOut = messages::WsBalanceEnquiryV2SoapOut;

    pub type WsBalanceEnquiryWalletSoapIn = messages::WsBalanceEnquiryWalletSoapIn;

    pub type WsBalanceEnquiryWalletSoapOut = messages::WsBalanceEnquiryWalletSoapOut;

    pub type WsCardStatementSoapIn = messages::WsCardStatementSoapIn;

    pub type WsCardStatementSoapOut = messages::WsCardStatementSoapOut;

    pub type WsCardStatementRepSoapIn = messages::WsCardStatementRepSoapIn;

    pub type WsCardStatementRepSoapOut = messages::WsCardStatementRepSoapOut;

    pub type WsCustomerEnquirySoapIn = messages::WsCustomerEnquirySoapIn;

    pub type WsCustomerEnquirySoapOut = messages::WsCustomerEnquirySoapOut;

    pub type WsCustomerEnquiryV2SoapIn = messages::WsCustomerEnquiryV2SoapIn;

    pub type WsCustomerEnquiryV2SoapOut = messages::WsCustomerEnquiryV2SoapOut;

    pub type WsUpdateCardholderDetailsSoapIn = messages::WsUpdateCardholderDetailsSoapIn;

    pub type WsUpdateCardholderDetailsSoapOut = messages::WsUpdateCardholderDetailsSoapOut;

    pub type WsUnLoadStatusChangeSoapIn = messages::WsUnLoadStatusChangeSoapIn;

    pub type WsUnLoadStatusChangeSoapOut = messages::WsUnLoadStatusChangeSoapOut;

    pub type WsActivateLoadSoapIn = messages::WsActivateLoadSoapIn;

    pub type WsActivateLoadSoapOut = messages::WsActivateLoadSoapOut;

    pub type WsBalanceAdjustmentSoapIn = messages::WsBalanceAdjustmentSoapIn;

    pub type WsBalanceAdjustmentSoapOut = messages::WsBalanceAdjustmentSoapOut;

    pub type WsExtendExpirySoapIn = messages::WsExtendExpirySoapIn;

    pub type WsExtendExpirySoapOut = messages::WsExtendExpirySoapOut;

    pub type WsTransactionVoidSoapIn = messages::WsTransactionVoidSoapIn;

    pub type WsTransactionVoidSoapOut = messages::WsTransactionVoidSoapOut;

    pub type WsCardHolderDetailsEnquirySoapIn = messages::WsCardHolderDetailsEnquirySoapIn;

    pub type WsCardHolderDetailsEnquirySoapOut = messages::WsCardHolderDetailsEnquirySoapOut;

    pub type WsCardHolderDetailsEnquiryV2SoapIn = messages::WsCardHolderDetailsEnquiryV2SoapIn;

    pub type WsCardHolderDetailsEnquiryV2SoapOut = messages::WsCardHolderDetailsEnquiryV2SoapOut;

    pub type WsPhoneActivationSoapIn = messages::WsPhoneActivationSoapIn;

    pub type WsPhoneActivationSoapOut = messages::WsPhoneActivationSoapOut;

    pub type WsBulkCreationSoapIn = messages::WsBulkCreationSoapIn;

    pub type WsBulkCreationSoapOut = messages::WsBulkCreationSoapOut;

    pub type WsBulkWalletCreationSoapIn = messages::WsBulkWalletCreationSoapIn;

    pub type WsBulkWalletCreationSoapOut = messages::WsBulkWalletCreationSoapOut;

    pub type WsWebServiceResultSoapIn = messages::WsWebServiceResultSoapIn;

    pub type WsWebServiceResultSoapOut = messages::WsWebServiceResultSoapOut;

    pub type WsGenericFeesSoapIn = messages::WsGenericFeesSoapIn;

    pub type WsGenericFeesSoapOut = messages::WsGenericFeesSoapOut;

    pub type WsCardBalEnqSoapIn = messages::WsCardBalEnqSoapIn;

    pub type WsCardBalEnqSoapOut = messages::WsCardBalEnqSoapOut;

    pub type WsPinControlSoapIn = messages::WsPinControlSoapIn;

    pub type WsPinControlSoapOut = messages::WsPinControlSoapOut;

    pub type WsCreateCardSoapIn = messages::WsCreateCardSoapIn;

    pub type WsCreateCardSoapOut = messages::WsCreateCardSoapOut;

    pub type WsCreateWalletSoapIn = messages::WsCreateWalletSoapIn;

    pub type WsCreateWalletSoapOut = messages::WsCreateWalletSoapOut;

    pub type WsRegenerateSoapIn = messages::WsRegenerateSoapIn;

    pub type WsRegenerateSoapOut = messages::WsRegenerateSoapOut;

    pub type WsConvertCardSoapIn = messages::WsConvertCardSoapIn;

    pub type WsConvertCardSoapOut = messages::WsConvertCardSoapOut;

    pub type WsChangeGroupsSoapIn = messages::WsChangeGroupsSoapIn;

    pub type WsChangeGroupsSoapOut = messages::WsChangeGroupsSoapOut;

    pub type WsCheckSoapIn = messages::WsCheckSoapIn;

    pub type WsCheckSoapOut = messages::WsCheckSoapOut;

    pub type WsSimpleCheckSoapIn = messages::WsSimpleCheckSoapIn;

    pub type WsSimpleCheckSoapOut = messages::WsSimpleCheckSoapOut;

    pub type WsClientFxSoapIn = messages::WsClientFxSoapIn;

    pub type WsClientFxSoapOut = messages::WsClientFxSoapOut;

    pub type WsLinkCardsSoapIn = messages::WsLinkCardsSoapIn;

    pub type WsLinkCardsSoapOut = messages::WsLinkCardsSoapOut;

    pub type WsListGroupSoapIn = messages::WsListGroupSoapIn;

    pub type WsListGroupSoapOut = messages::WsListGroupSoapOut;

    pub type WsListProductsSoapIn = messages::WsListProductsSoapIn;

    pub type WsListProductsSoapOut = messages::WsListProductsSoapOut;

    pub type WsGetCardRequestSoapIn = messages::WsGetCardRequestSoapIn;

    pub type WsGetCardRequestSoapOut = messages::WsGetCardRequestSoapOut;

    pub type WsGetCardRequestStatusSoapIn = messages::WsGetCardRequestStatusSoapIn;

    pub type WsGetCardRequestStatusSoapOut = messages::WsGetCardRequestStatusSoapOut;

    pub type WsCardAcceptorWhiteListSoapIn = messages::WsCardAcceptorWhiteListSoapIn;

    pub type WsCardAcceptorWhiteListSoapOut = messages::WsCardAcceptorWhiteListSoapOut;

    pub type WsCardAcceptorBlackListSoapIn = messages::WsCardAcceptorBlackListSoapIn;

    pub type WsCardAcceptorBlackListSoapOut = messages::WsCardAcceptorBlackListSoapOut;

    pub type WsSendMessageSoapIn = messages::WsSendMessageSoapIn;

    pub type WsSendMessageSoapOut = messages::WsSendMessageSoapOut;

    pub type WsMVCLoadSoapIn = messages::WsMVCLoadSoapIn;

    pub type WsMVCLoadSoapOut = messages::WsMVCLoadSoapOut;

    pub type WsListPendingFeesSoapIn = messages::WsListPendingFeesSoapIn;

    pub type WsListPendingFeesSoapOut = messages::WsListPendingFeesSoapOut;

    pub type WsWebServiceResultV2SoapIn = messages::WsWebServiceResultV2SoapIn;

    pub type WsWebServiceResultV2SoapOut = messages::WsWebServiceResultV2SoapOut;

    pub type WsGetPasscodeSoapIn = messages::WsGetPasscodeSoapIn;

    pub type WsGetPasscodeSoapOut = messages::WsGetPasscodeSoapOut;

    pub type WsGetCardExpireSoonSoapIn = messages::WsGetCardExpireSoonSoapIn;

    pub type WsGetCardExpireSoonSoapOut = messages::WsGetCardExpireSoonSoapOut;

    pub type WsSendCardFilesSoapIn = messages::WsSendCardFilesSoapIn;

    pub type WsSendCardFilesSoapOut = messages::WsSendCardFilesSoapOut;

    pub type WsSafeReportsSoapIn = messages::WsSafeReportsSoapIn;

    pub type WsSafeReportsSoapOut = messages::WsSafeReportsSoapOut;

    pub type WsRegenerateWalletSoapIn = messages::WsRegenerateWalletSoapIn;

    pub type WsRegenerateWalletSoapOut = messages::WsRegenerateWalletSoapOut;

    pub type WsUpdateLoadSourceSoapIn = messages::WsUpdateLoadSourceSoapIn;

    pub type WsUpdateLoadSourceSoapOut = messages::WsUpdateLoadSourceSoapOut;

    pub type WsMVCUnloadSoapIn = messages::WsMVCUnloadSoapIn;

    pub type WsMVCUnloadSoapOut = messages::WsMVCUnloadSoapOut;

    pub type WsActivateMVCLoadSoapIn = messages::WsActivateMVCLoadSoapIn;

    pub type WsActivateMVCLoadSoapOut = messages::WsActivateMVCLoadSoapOut;

    pub type WsRenewCardSoapIn = messages::WsRenewCardSoapIn;

    pub type WsRenewCardSoapOut = messages::WsRenewCardSoapOut;

    pub type WsResetAccumulatorSoapIn = messages::WsResetAccumulatorSoapIn;

    pub type WsResetAccumulatorSoapOut = messages::WsResetAccumulatorSoapOut;

    pub type WsEnrolSoapIn = messages::WsEnrolSoapIn;

    pub type WsEnrolSoapOut = messages::WsEnrolSoapOut;

    pub type WsGiftCardActivateSoapIn = messages::WsGiftCardActivateSoapIn;

    pub type WsGiftCardActivateSoapOut = messages::WsGiftCardActivateSoapOut;

    pub type WsGiftCardLoadSoapIn = messages::WsGiftCardLoadSoapIn;

    pub type WsGiftCardLoadSoapOut = messages::WsGiftCardLoadSoapOut;

    pub type WsGiftCardUnLoadSoapIn = messages::WsGiftCardUnLoadSoapIn;

    pub type WsGiftCardUnLoadSoapOut = messages::WsGiftCardUnLoadSoapOut;

    pub type WsGiftCardStatusChangeSoapIn = messages::WsGiftCardStatusChangeSoapIn;

    pub type WsGiftCardStatusChangeSoapOut = messages::WsGiftCardStatusChangeSoapOut;

    pub type WsGiftCardEnquirySoapIn = messages::WsGiftCardEnquirySoapIn;

    pub type WsGiftCardEnquirySoapOut = messages::WsGiftCardEnquirySoapOut;

    pub type WsGiftCardBalanceTransferSoapIn = messages::WsGiftCardBalanceTransferSoapIn;

    pub type WsGiftCardBalanceTransferSoapOut = messages::WsGiftCardBalanceTransferSoapOut;

    pub type WsGiftCardBalanceEnquirySoapIn = messages::WsGiftCardBalanceEnquirySoapIn;

    pub type WsGiftCardBalanceEnquirySoapOut = messages::WsGiftCardBalanceEnquirySoapOut;

    pub type WsGiftCardCardStatementSoapIn = messages::WsGiftCardCardStatementSoapIn;

    pub type WsGiftCardCardStatementSoapOut = messages::WsGiftCardCardStatementSoapOut;

    pub type WsGiftCardUpdateCardholderDetailsSoapIn =
        messages::WsGiftCardUpdateCardholderDetailsSoapIn;

    pub type WsGiftCardUpdateCardholderDetailsSoapOut =
        messages::WsGiftCardUpdateCardholderDetailsSoapOut;

    pub type WsGiftCardUnLoadStatusChangeSoapIn = messages::WsGiftCardUnLoadStatusChangeSoapIn;

    pub type WsGiftCardUnLoadStatusChangeSoapOut = messages::WsGiftCardUnLoadStatusChangeSoapOut;

    pub type WsGiftCardActivateLoadSoapIn = messages::WsGiftCardActivateLoadSoapIn;

    pub type WsGiftCardActivateLoadSoapOut = messages::WsGiftCardActivateLoadSoapOut;

    pub type WsGiftCardBalanceAdjustmentSoapIn = messages::WsGiftCardBalanceAdjustmentSoapIn;

    pub type WsGiftCardBalanceAdjustmentSoapOut = messages::WsGiftCardBalanceAdjustmentSoapOut;

    pub type WsGiftCardExtendExpirySoapIn = messages::WsGiftCardExtendExpirySoapIn;

    pub type WsGiftCardExtendExpirySoapOut = messages::WsGiftCardExtendExpirySoapOut;

    pub type WsGiftCardTransactionVoidSoapIn = messages::WsGiftCardTransactionVoidSoapIn;

    pub type WsGiftCardTransactionVoidSoapOut = messages::WsGiftCardTransactionVoidSoapOut;

    pub type WsGiftCardCardHolderDetailsEnquirySoapIn =
        messages::WsGiftCardCardHolderDetailsEnquirySoapIn;

    pub type WsGiftCardCardHolderDetailsEnquirySoapOut =
        messages::WsGiftCardCardHolderDetailsEnquirySoapOut;

    pub type WsGiftCardPhoneActivationSoapIn = messages::WsGiftCardPhoneActivationSoapIn;

    pub type WsGiftCardPhoneActivationSoapOut = messages::WsGiftCardPhoneActivationSoapOut;

    pub type WsGiftCardBulkCreationSoapIn = messages::WsGiftCardBulkCreationSoapIn;

    pub type WsGiftCardBulkCreationSoapOut = messages::WsGiftCardBulkCreationSoapOut;

    pub type WsGiftCardWebServiceResultSoapIn = messages::WsGiftCardWebServiceResultSoapIn;

    pub type WsGiftCardWebServiceResultSoapOut = messages::WsGiftCardWebServiceResultSoapOut;

    pub type WsGiftCardGenericFeesSoapIn = messages::WsGiftCardGenericFeesSoapIn;

    pub type WsGiftCardGenericFeesSoapOut = messages::WsGiftCardGenericFeesSoapOut;

    pub type WsGiftCardPinControlSoapIn = messages::WsGiftCardPinControlSoapIn;

    pub type WsGiftCardPinControlSoapOut = messages::WsGiftCardPinControlSoapOut;

    pub type WsGiftCardUpdateLoadSourceSoapIn = messages::WsGiftCardUpdateLoadSourceSoapIn;

    pub type WsGiftCardUpdateLoadSourceSoapOut = messages::WsGiftCardUpdateLoadSourceSoapOut;

    pub type WsGiftCardActivateLoadProductTpyeCPSoapIn =
        messages::WsGiftCardActivateLoadProductTpyeCPSoapIn;

    pub type WsGiftCardActivateLoadProductTpyeCPSoapOut =
        messages::WsGiftCardActivateLoadProductTpyeCPSoapOut;

    pub type WsCardTransactionXMLSoapIn = messages::WsCardTransactionXMLSoapIn;

    pub type WsCardTransactionXMLSoapOut = messages::WsCardTransactionXMLSoapOut;

    pub type WsCardChangeGroupsSoapIn = messages::WsCardChangeGroupsSoapIn;

    pub type WsCardChangeGroupsSoapOut = messages::WsCardChangeGroupsSoapOut;

    pub type WsCardChangeCardacceptorListSoapIn = messages::WsCardChangeCardacceptorListSoapIn;

    pub type WsCardChangeCardacceptorListSoapOut = messages::WsCardChangeCardacceptorListSoapOut;

    pub type WsChangeCardacceptorListSoapIn = messages::WsChangeCardacceptorListSoapIn;

    pub type WsChangeCardacceptorListSoapOut = messages::WsChangeCardacceptorListSoapOut;

    pub type WsAddressMatchCheckingSoapIn = messages::WsAddressMatchCheckingSoapIn;

    pub type WsAddressMatchCheckingSoapOut = messages::WsAddressMatchCheckingSoapOut;

    pub type WsLicenseVerificationSoapIn = messages::WsLicenseVerificationSoapIn;

    pub type WsLicenseVerificationSoapOut = messages::WsLicenseVerificationSoapOut;

    pub type WsPassportVerificationSoapIn = messages::WsPassportVerificationSoapIn;

    pub type WsPassportVerificationSoapOut = messages::WsPassportVerificationSoapOut;

    pub type WsSanctionsPEPCheckSoapIn = messages::WsSanctionsPEPCheckSoapIn;

    pub type WsSanctionsPEPCheckSoapOut = messages::WsSanctionsPEPCheckSoapOut;

    pub type WsSanctionsPEPCheckV2SoapIn = messages::WsSanctionsPEPCheckV2SoapIn;

    pub type WsSanctionsPEPCheckV2SoapOut = messages::WsSanctionsPEPCheckV2SoapOut;

    pub type WsListSanctionsPEPSoapIn = messages::WsListSanctionsPEPSoapIn;

    pub type WsListSanctionsPEPSoapOut = messages::WsListSanctionsPEPSoapOut;

    pub type WsListSanctionsPEPMatchesSoapIn = messages::WsListSanctionsPEPMatchesSoapIn;

    pub type WsListSanctionsPEPMatchesSoapOut = messages::WsListSanctionsPEPMatchesSoapOut;

    pub type WsUpdateSanctionsPEPMatchesSoapIn = messages::WsUpdateSanctionsPEPMatchesSoapIn;

    pub type WsUpdateSanctionsPEPMatchesSoapOut = messages::WsUpdateSanctionsPEPMatchesSoapOut;

    pub type WsCreateCardV2SoapIn = messages::WsCreateCardV2SoapIn;

    pub type WsCreateCardV2SoapOut = messages::WsCreateCardV2SoapOut;

    pub type WsBankingReturnBankDetailsFromTokenSoapIn =
        messages::WsBankingReturnBankDetailsFromTokenSoapIn;

    pub type WsBankingReturnBankDetailsFromTokenSoapOut =
        messages::WsBankingReturnBankDetailsFromTokenSoapOut;

    pub type WsBankingChangeAccountBankingFeaturesStatusSoapIn =
        messages::WsBankingChangeAccountBankingFeaturesStatusSoapIn;

    pub type WsBankingChangeAccountBankingFeaturesStatusSoapOut =
        messages::WsBankingChangeAccountBankingFeaturesStatusSoapOut;

    pub type WsBankingTransferFundsSoapIn = messages::WsBankingTransferFundsSoapIn;

    pub type WsBankingTransferFundsSoapOut = messages::WsBankingTransferFundsSoapOut;
}

pub mod bindings {
    use super::*;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingUpdateBankingEnabledCardSoapIn {
        #[yaserde(rename = "Ws_Banking_UpdateBankingEnabledCard", default)]
        pub body: ports::WsBankingUpdateBankingEnabledCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingUpdateBankingEnabledCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingUpdateBankingEnabledCardSoapIn,
    }

    impl WsBankingUpdateBankingEnabledCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingUpdateBankingEnabledCardSoapIn) -> Self {
            WsBankingUpdateBankingEnabledCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingUpdateBankingEnabledCardSoapOut {
        #[yaserde(rename = "Ws_BankingUpdateBankingEnabledCardResponse", default)]
        pub body: ports::WsBankingUpdateBankingEnabledCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingUpdateBankingEnabledCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingUpdateBankingEnabledCardSoapOut,
    }

    impl WsBankingUpdateBankingEnabledCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingUpdateBankingEnabledCardSoapOut) -> Self {
            WsBankingUpdateBankingEnabledCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingStatusQueryBankingEnabledCardSoapIn {
        #[yaserde(rename = "Ws_Banking_StatusQueryBankingEnabledCard", default)]
        pub body: ports::WsBankingStatusQueryBankingEnabledCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingStatusQueryBankingEnabledCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingStatusQueryBankingEnabledCardSoapIn,
    }

    impl WsBankingStatusQueryBankingEnabledCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingStatusQueryBankingEnabledCardSoapIn) -> Self {
            WsBankingStatusQueryBankingEnabledCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingStatusQueryBankingEnabledCardSoapOut {
        #[yaserde(rename = "Ws_BankingStatusQueryBankingEnabledCardResponse", default)]
        pub body: ports::WsBankingStatusQueryBankingEnabledCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingStatusQueryBankingEnabledCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingStatusQueryBankingEnabledCardSoapOut,
    }

    impl WsBankingStatusQueryBankingEnabledCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingStatusQueryBankingEnabledCardSoapOut) -> Self {
            WsBankingStatusQueryBankingEnabledCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingCreateCustomerSoapIn {
        #[yaserde(rename = "Ws_Banking_CreateCustomer", default)]
        pub body: ports::WsBankingCreateCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingCreateCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingCreateCustomerSoapIn,
    }

    impl WsBankingCreateCustomerSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingCreateCustomerSoapIn) -> Self {
            WsBankingCreateCustomerSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingCreateCustomerSoapOut {
        #[yaserde(rename = "Ws_BankingCreateCustomerResponse", default)]
        pub body: ports::WsBankingCreateCustomerSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingCreateCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingCreateCustomerSoapOut,
    }

    impl WsBankingCreateCustomerSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingCreateCustomerSoapOut) -> Self {
            WsBankingCreateCustomerSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingRegisterNotificationSoapIn {
        #[yaserde(rename = "Ws_Banking_RegisterNotification", default)]
        pub body: ports::WsBankingRegisterNotificationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingRegisterNotificationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingRegisterNotificationSoapIn,
    }

    impl WsBankingRegisterNotificationSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingRegisterNotificationSoapIn) -> Self {
            WsBankingRegisterNotificationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingRegisterNotificationSoapOut {
        #[yaserde(rename = "Ws_BankingRegisterNotificationResponse", default)]
        pub body: ports::WsBankingRegisterNotificationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingRegisterNotificationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingRegisterNotificationSoapOut,
    }

    impl WsBankingRegisterNotificationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingRegisterNotificationSoapOut) -> Self {
            WsBankingRegisterNotificationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingAccountModulusCheckSoapIn {
        #[yaserde(rename = "Ws_Banking_AccountModulusCheck", default)]
        pub body: ports::WsBankingAccountModulusCheckSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingAccountModulusCheckSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingAccountModulusCheckSoapIn,
    }

    impl WsBankingAccountModulusCheckSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingAccountModulusCheckSoapIn) -> Self {
            WsBankingAccountModulusCheckSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingAccountModulusCheckSoapOut {
        #[yaserde(rename = "Ws_BankingAccountModulusCheckResponse", default)]
        pub body: ports::WsBankingAccountModulusCheckSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingAccountModulusCheckSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingAccountModulusCheckSoapOut,
    }

    impl WsBankingAccountModulusCheckSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingAccountModulusCheckSoapOut) -> Self {
            WsBankingAccountModulusCheckSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn {
        #[yaserde(
            rename = "Ws_Banking_GetDirectDebitInstructionsBankingEnabledCard",
            default
        )]
        pub body: ports::WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn,
    }

    impl WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingGetDirectDebitInstructionsBankingEnabledCardSoapIn) -> Self {
            WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut {
        #[yaserde(
            rename = "Ws_BankingGetDirectDebitInstructionsBankingEnabledCardResponse",
            default
        )]
        pub body: ports::WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut,
    }

    impl WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOut) -> Self {
            WsBankingGetDirectDebitInstructionsBankingEnabledCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingCancelDirectDebitBankingEnabledCardSoapIn {
        #[yaserde(rename = "Ws_Banking_CancelDirectDebitBankingEnabledCard", default)]
        pub body: ports::WsBankingCancelDirectDebitBankingEnabledCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingCancelDirectDebitBankingEnabledCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingCancelDirectDebitBankingEnabledCardSoapIn,
    }

    impl WsBankingCancelDirectDebitBankingEnabledCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingCancelDirectDebitBankingEnabledCardSoapIn) -> Self {
            WsBankingCancelDirectDebitBankingEnabledCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingCancelDirectDebitBankingEnabledCardSoapOut {
        #[yaserde(
            rename = "Ws_BankingCancelDirectDebitBankingEnabledCardResponse",
            default
        )]
        pub body: ports::WsBankingCancelDirectDebitBankingEnabledCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingCancelDirectDebitBankingEnabledCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingCancelDirectDebitBankingEnabledCardSoapOut,
    }

    impl WsBankingCancelDirectDebitBankingEnabledCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingCancelDirectDebitBankingEnabledCardSoapOut) -> Self {
            WsBankingCancelDirectDebitBankingEnabledCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingGetPendingDirectDebitsSoapIn {
        #[yaserde(rename = "Ws_Banking_GetPendingDirectDebits", default)]
        pub body: ports::WsBankingGetPendingDirectDebitsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingGetPendingDirectDebitsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingGetPendingDirectDebitsSoapIn,
    }

    impl WsBankingGetPendingDirectDebitsSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingGetPendingDirectDebitsSoapIn) -> Self {
            WsBankingGetPendingDirectDebitsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingGetPendingDirectDebitsSoapOut {
        #[yaserde(rename = "Ws_BankingGetPendingDirectDebitsResponse", default)]
        pub body: ports::WsBankingGetPendingDirectDebitsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingGetPendingDirectDebitsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingGetPendingDirectDebitsSoapOut,
    }

    impl WsBankingGetPendingDirectDebitsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingGetPendingDirectDebitsSoapOut) -> Self {
            WsBankingGetPendingDirectDebitsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingCardStatementV2SoapIn {
        #[yaserde(rename = "Ws_Banking_Card_Statement_V2", default)]
        pub body: ports::WsBankingCardStatementV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingCardStatementV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingCardStatementV2SoapIn,
    }

    impl WsBankingCardStatementV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingCardStatementV2SoapIn) -> Self {
            WsBankingCardStatementV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingCardStatementV2SoapOut {
        #[yaserde(rename = "Ws_BankingCardStatementV2Response", default)]
        pub body: ports::WsBankingCardStatementV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingCardStatementV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingCardStatementV2SoapOut,
    }

    impl WsBankingCardStatementV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingCardStatementV2SoapOut) -> Self {
            WsBankingCardStatementV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsInsert3DSecureDetailsSoapIn {
        #[yaserde(rename = "Ws_Insert3DSecureDetails", default)]
        pub body: ports::WsInsert3DSecureDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsInsert3DSecureDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsInsert3DSecureDetailsSoapIn,
    }

    impl WsInsert3DSecureDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWsInsert3DSecureDetailsSoapIn) -> Self {
            WsInsert3DSecureDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsInsert3DSecureDetailsSoapOut {
        #[yaserde(rename = "Ws_Insert3DSecureDetailsResponse", default)]
        pub body: ports::WsInsert3DSecureDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsInsert3DSecureDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsInsert3DSecureDetailsSoapOut,
    }

    impl WsInsert3DSecureDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsInsert3DSecureDetailsSoapOut) -> Self {
            WsInsert3DSecureDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdate3DSecureDetailsSoapIn {
        #[yaserde(rename = "Ws_Update3DSecureDetails", default)]
        pub body: ports::WsUpdate3DSecureDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdate3DSecureDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdate3DSecureDetailsSoapIn,
    }

    impl WsUpdate3DSecureDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWsUpdate3DSecureDetailsSoapIn) -> Self {
            WsUpdate3DSecureDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdate3DSecureDetailsSoapOut {
        #[yaserde(rename = "Ws_Update3DSecureDetailsResponse", default)]
        pub body: ports::WsUpdate3DSecureDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdate3DSecureDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdate3DSecureDetailsSoapOut,
    }

    impl WsUpdate3DSecureDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUpdate3DSecureDetailsSoapOut) -> Self {
            WsUpdate3DSecureDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateLastModifiedTypeSoapIn {
        #[yaserde(rename = "Ws_UpdateLastModifiedType", default)]
        pub body: ports::WsUpdateLastModifiedTypeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateLastModifiedTypeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateLastModifiedTypeSoapIn,
    }

    impl WsUpdateLastModifiedTypeSoapInSoapEnvelope {
        pub fn new(body: SoapWsUpdateLastModifiedTypeSoapIn) -> Self {
            WsUpdateLastModifiedTypeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateLastModifiedTypeSoapOut {
        #[yaserde(rename = "Ws_UpdateLastModifiedTypeResponse", default)]
        pub body: ports::WsUpdateLastModifiedTypeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateLastModifiedTypeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateLastModifiedTypeSoapOut,
    }

    impl WsUpdateLastModifiedTypeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUpdateLastModifiedTypeSoapOut) -> Self {
            WsUpdateLastModifiedTypeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsDelete3DSecureDetailsSoapIn {
        #[yaserde(rename = "Ws_Delete3DSecureDetails", default)]
        pub body: ports::WsDelete3DSecureDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsDelete3DSecureDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsDelete3DSecureDetailsSoapIn,
    }

    impl WsDelete3DSecureDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWsDelete3DSecureDetailsSoapIn) -> Self {
            WsDelete3DSecureDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsDelete3DSecureDetailsSoapOut {
        #[yaserde(rename = "Ws_Delete3DSecureDetailsResponse", default)]
        pub body: ports::WsDelete3DSecureDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsDelete3DSecureDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsDelete3DSecureDetailsSoapOut,
    }

    impl WsDelete3DSecureDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsDelete3DSecureDetailsSoapOut) -> Self {
            WsDelete3DSecureDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsQuery3DSecureDetailsSoapIn {
        #[yaserde(rename = "Ws_Query3DSecureDetails", default)]
        pub body: ports::WsQuery3DSecureDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsQuery3DSecureDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsQuery3DSecureDetailsSoapIn,
    }

    impl WsQuery3DSecureDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWsQuery3DSecureDetailsSoapIn) -> Self {
            WsQuery3DSecureDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsQuery3DSecureDetailsSoapOut {
        #[yaserde(rename = "Ws_Query3DSecureDetailsResponse", default)]
        pub body: ports::WsQuery3DSecureDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsQuery3DSecureDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsQuery3DSecureDetailsSoapOut,
    }

    impl WsQuery3DSecureDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsQuery3DSecureDetailsSoapOut) -> Self {
            WsQuery3DSecureDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGPSLockUnlockSoapIn {
        #[yaserde(rename = "Ws_GPS_Lock_Unlock", default)]
        pub body: ports::WsGPSLockUnlockSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGPSLockUnlockSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGPSLockUnlockSoapIn,
    }

    impl WsGPSLockUnlockSoapInSoapEnvelope {
        pub fn new(body: SoapWsGPSLockUnlockSoapIn) -> Self {
            WsGPSLockUnlockSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGPSLockUnlockSoapOut {
        #[yaserde(rename = "Ws_GPSLockUnlockResponse", default)]
        pub body: ports::WsGPSLockUnlockSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGPSLockUnlockSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGPSLockUnlockSoapOut,
    }

    impl WsGPSLockUnlockSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGPSLockUnlockSoapOut) -> Self {
            WsGPSLockUnlockSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsVerificationRequestSoapIn {
        #[yaserde(rename = "WS_VerificationRequest", default)]
        pub body: ports::WsVerificationRequestSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsVerificationRequestSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsVerificationRequestSoapIn,
    }

    impl WsVerificationRequestSoapInSoapEnvelope {
        pub fn new(body: SoapWsVerificationRequestSoapIn) -> Self {
            WsVerificationRequestSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsVerificationRequestSoapOut {
        #[yaserde(rename = "Ws_VerificationRequestResponse", default)]
        pub body: ports::WsVerificationRequestSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsVerificationRequestSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsVerificationRequestSoapOut,
    }

    impl WsVerificationRequestSoapOutSoapEnvelope {
        pub fn new(body: SoapWsVerificationRequestSoapOut) -> Self {
            WsVerificationRequestSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsAddUpDelCredentialsSoapIn {
        #[yaserde(rename = "Ws_AddUpDelCredentials", default)]
        pub body: ports::WsAddUpDelCredentialsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsAddUpDelCredentialsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsAddUpDelCredentialsSoapIn,
    }

    impl WsAddUpDelCredentialsSoapInSoapEnvelope {
        pub fn new(body: SoapWsAddUpDelCredentialsSoapIn) -> Self {
            WsAddUpDelCredentialsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsAddUpDelCredentialsSoapOut {
        #[yaserde(rename = "Ws_AddUpDelCredentialsResponse", default)]
        pub body: ports::WsAddUpDelCredentialsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsAddUpDelCredentialsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsAddUpDelCredentialsSoapOut,
    }

    impl WsAddUpDelCredentialsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsAddUpDelCredentialsSoapOut) -> Self {
            WsAddUpDelCredentialsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWs3DSAddUpDelDetailsSoapIn {
        #[yaserde(rename = "Ws_3DS_AddUpDelDetails", default)]
        pub body: ports::Ws3DSAddUpDelDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct Ws3DSAddUpDelDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWs3DSAddUpDelDetailsSoapIn,
    }

    impl Ws3DSAddUpDelDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWs3DSAddUpDelDetailsSoapIn) -> Self {
            Ws3DSAddUpDelDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWs3DSAddUpDelDetailsSoapOut {
        #[yaserde(rename = "Ws_3DSAddUpDelDetailsResponse", default)]
        pub body: ports::Ws3DSAddUpDelDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct Ws3DSAddUpDelDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWs3DSAddUpDelDetailsSoapOut,
    }

    impl Ws3DSAddUpDelDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWs3DSAddUpDelDetailsSoapOut) -> Self {
            Ws3DSAddUpDelDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceUpdateSoapIn {
        #[yaserde(rename = "Ws_BalanceUpdate", default)]
        pub body: ports::WsBalanceUpdateSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceUpdateSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceUpdateSoapIn,
    }

    impl WsBalanceUpdateSoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceUpdateSoapIn) -> Self {
            WsBalanceUpdateSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceUpdateSoapOut {
        #[yaserde(rename = "Ws_BalanceUpdateResponse", default)]
        pub body: ports::WsBalanceUpdateSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceUpdateSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceUpdateSoapOut,
    }

    impl WsBalanceUpdateSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceUpdateSoapOut) -> Self {
            WsBalanceUpdateSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPaymentTokenGetSoapIn {
        #[yaserde(rename = "Ws_Payment_Token_Get", default)]
        pub body: ports::WsPaymentTokenGetSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPaymentTokenGetSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPaymentTokenGetSoapIn,
    }

    impl WsPaymentTokenGetSoapInSoapEnvelope {
        pub fn new(body: SoapWsPaymentTokenGetSoapIn) -> Self {
            WsPaymentTokenGetSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPaymentTokenGetSoapOut {
        #[yaserde(rename = "Ws_PaymentTokenGetResponse", default)]
        pub body: ports::WsPaymentTokenGetSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPaymentTokenGetSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPaymentTokenGetSoapOut,
    }

    impl WsPaymentTokenGetSoapOutSoapEnvelope {
        pub fn new(body: SoapWsPaymentTokenGetSoapOut) -> Self {
            WsPaymentTokenGetSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPaymentTokenStatusChangeSoapIn {
        #[yaserde(rename = "Ws_Payment_Token_StatusChange", default)]
        pub body: ports::WsPaymentTokenStatusChangeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPaymentTokenStatusChangeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPaymentTokenStatusChangeSoapIn,
    }

    impl WsPaymentTokenStatusChangeSoapInSoapEnvelope {
        pub fn new(body: SoapWsPaymentTokenStatusChangeSoapIn) -> Self {
            WsPaymentTokenStatusChangeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPaymentTokenStatusChangeSoapOut {
        #[yaserde(rename = "Ws_PaymentTokenStatusChangeResponse", default)]
        pub body: ports::WsPaymentTokenStatusChangeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPaymentTokenStatusChangeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPaymentTokenStatusChangeSoapOut,
    }

    impl WsPaymentTokenStatusChangeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsPaymentTokenStatusChangeSoapOut) -> Self {
            WsPaymentTokenStatusChangeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsActivateSoapIn {
        #[yaserde(rename = "Ws_Activate", default)]
        pub body: ports::WsActivateSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsActivateSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsActivateSoapIn,
    }

    impl WsActivateSoapInSoapEnvelope {
        pub fn new(body: SoapWsActivateSoapIn) -> Self {
            WsActivateSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsActivateSoapOut {
        #[yaserde(rename = "Ws_ActivateResponse", default)]
        pub body: ports::WsActivateSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsActivateSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsActivateSoapOut,
    }

    impl WsActivateSoapOutSoapEnvelope {
        pub fn new(body: SoapWsActivateSoapOut) -> Self {
            WsActivateSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsLoadSoapIn {
        #[yaserde(rename = "Ws_Load", default)]
        pub body: ports::WsLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsLoadSoapIn,
    }

    impl WsLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsLoadSoapIn) -> Self {
            WsLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsLoadSoapOut {
        #[yaserde(rename = "Ws_LoadResponse", default)]
        pub body: ports::WsLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsLoadSoapOut,
    }

    impl WsLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsLoadSoapOut) -> Self {
            WsLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUnLoadSoapIn {
        #[yaserde(rename = "Ws_UnLoad", default)]
        pub body: ports::WsUnLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUnLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUnLoadSoapIn,
    }

    impl WsUnLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsUnLoadSoapIn) -> Self {
            WsUnLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUnLoadSoapOut {
        #[yaserde(rename = "Ws_UnLoadResponse", default)]
        pub body: ports::WsUnLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUnLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUnLoadSoapOut,
    }

    impl WsUnLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUnLoadSoapOut) -> Self {
            WsUnLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsStatusChangeSoapIn {
        #[yaserde(rename = "Ws_StatusChange", default)]
        pub body: ports::WsStatusChangeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsStatusChangeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsStatusChangeSoapIn,
    }

    impl WsStatusChangeSoapInSoapEnvelope {
        pub fn new(body: SoapWsStatusChangeSoapIn) -> Self {
            WsStatusChangeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsStatusChangeSoapOut {
        #[yaserde(rename = "Ws_StatusChangeResponse", default)]
        pub body: ports::WsStatusChangeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsStatusChangeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsStatusChangeSoapOut,
    }

    impl WsStatusChangeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsStatusChangeSoapOut) -> Self {
            WsStatusChangeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsEnquirySoapIn {
        #[yaserde(rename = "Ws_Enquiry", default)]
        pub body: ports::WsEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsEnquirySoapIn,
    }

    impl WsEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsEnquirySoapIn) -> Self {
            WsEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsEnquirySoapOut {
        #[yaserde(rename = "Ws_EnquiryResponse", default)]
        pub body: ports::WsEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsEnquirySoapOut,
    }

    impl WsEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsEnquirySoapOut) -> Self {
            WsEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceTransferSoapIn {
        #[yaserde(rename = "Ws_BalanceTransfer", default)]
        pub body: ports::WsBalanceTransferSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceTransferSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceTransferSoapIn,
    }

    impl WsBalanceTransferSoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceTransferSoapIn) -> Self {
            WsBalanceTransferSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceTransferSoapOut {
        #[yaserde(rename = "Ws_BalanceTransferResponse", default)]
        pub body: ports::WsBalanceTransferSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceTransferSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceTransferSoapOut,
    }

    impl WsBalanceTransferSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceTransferSoapOut) -> Self {
            WsBalanceTransferSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquirySoapIn {
        #[yaserde(rename = "Ws_Balance_Enquiry", default)]
        pub body: ports::WsBalanceEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquirySoapIn,
    }

    impl WsBalanceEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquirySoapIn) -> Self {
            WsBalanceEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquirySoapOut {
        #[yaserde(rename = "Ws_BalanceEnquiryResponse", default)]
        pub body: ports::WsBalanceEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquirySoapOut,
    }

    impl WsBalanceEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquirySoapOut) -> Self {
            WsBalanceEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquiryRepSoapIn {
        #[yaserde(rename = "Ws_Balance_Enquiry_Rep", default)]
        pub body: ports::WsBalanceEnquiryRepSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquiryRepSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquiryRepSoapIn,
    }

    impl WsBalanceEnquiryRepSoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquiryRepSoapIn) -> Self {
            WsBalanceEnquiryRepSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquiryRepSoapOut {
        #[yaserde(rename = "Ws_BalanceEnquiryRepResponse", default)]
        pub body: ports::WsBalanceEnquiryRepSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquiryRepSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquiryRepSoapOut,
    }

    impl WsBalanceEnquiryRepSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquiryRepSoapOut) -> Self {
            WsBalanceEnquiryRepSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquiryV2SoapIn {
        #[yaserde(rename = "Ws_Balance_Enquiry_V2", default)]
        pub body: ports::WsBalanceEnquiryV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquiryV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquiryV2SoapIn,
    }

    impl WsBalanceEnquiryV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquiryV2SoapIn) -> Self {
            WsBalanceEnquiryV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquiryV2SoapOut {
        #[yaserde(rename = "Ws_Balance_Enquiry_V2Response", default)]
        pub body: ports::WsBalanceEnquiryV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquiryV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquiryV2SoapOut,
    }

    impl WsBalanceEnquiryV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquiryV2SoapOut) -> Self {
            WsBalanceEnquiryV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquiryWalletSoapIn {
        #[yaserde(rename = "Ws_Balance_Enquiry_Wallet", default)]
        pub body: ports::WsBalanceEnquiryWalletSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquiryWalletSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquiryWalletSoapIn,
    }

    impl WsBalanceEnquiryWalletSoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquiryWalletSoapIn) -> Self {
            WsBalanceEnquiryWalletSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceEnquiryWalletSoapOut {
        #[yaserde(rename = "Ws_BalanceEnquiryWalletResponse", default)]
        pub body: ports::WsBalanceEnquiryWalletSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceEnquiryWalletSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceEnquiryWalletSoapOut,
    }

    impl WsBalanceEnquiryWalletSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceEnquiryWalletSoapOut) -> Self {
            WsBalanceEnquiryWalletSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardStatementSoapIn {
        #[yaserde(rename = "Ws_Card_Statement", default)]
        pub body: ports::WsCardStatementSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardStatementSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardStatementSoapIn,
    }

    impl WsCardStatementSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardStatementSoapIn) -> Self {
            WsCardStatementSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardStatementSoapOut {
        #[yaserde(rename = "Ws_Card_StatementResponse", default)]
        pub body: ports::WsCardStatementSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardStatementSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardStatementSoapOut,
    }

    impl WsCardStatementSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardStatementSoapOut) -> Self {
            WsCardStatementSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardStatementRepSoapIn {
        #[yaserde(rename = "Ws_Card_Statement_Rep", default)]
        pub body: ports::WsCardStatementRepSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardStatementRepSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardStatementRepSoapIn,
    }

    impl WsCardStatementRepSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardStatementRepSoapIn) -> Self {
            WsCardStatementRepSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardStatementRepSoapOut {
        #[yaserde(rename = "Ws_CardStatementRepResponse", default)]
        pub body: ports::WsCardStatementRepSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardStatementRepSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardStatementRepSoapOut,
    }

    impl WsCardStatementRepSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardStatementRepSoapOut) -> Self {
            WsCardStatementRepSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCustomerEnquirySoapIn {
        #[yaserde(rename = "Ws_Customer_Enquiry", default)]
        pub body: ports::WsCustomerEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCustomerEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCustomerEnquirySoapIn,
    }

    impl WsCustomerEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsCustomerEnquirySoapIn) -> Self {
            WsCustomerEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCustomerEnquirySoapOut {
        #[yaserde(rename = "Ws_CustomerEnquiryResponse", default)]
        pub body: ports::WsCustomerEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCustomerEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCustomerEnquirySoapOut,
    }

    impl WsCustomerEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsCustomerEnquirySoapOut) -> Self {
            WsCustomerEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCustomerEnquiryV2SoapIn {
        #[yaserde(rename = "Ws_Customer_Enquiry_V2", default)]
        pub body: ports::WsCustomerEnquiryV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCustomerEnquiryV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCustomerEnquiryV2SoapIn,
    }

    impl WsCustomerEnquiryV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsCustomerEnquiryV2SoapIn) -> Self {
            WsCustomerEnquiryV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCustomerEnquiryV2SoapOut {
        #[yaserde(rename = "Ws_CustomerEnquiryV2Response", default)]
        pub body: ports::WsCustomerEnquiryV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCustomerEnquiryV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCustomerEnquiryV2SoapOut,
    }

    impl WsCustomerEnquiryV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsCustomerEnquiryV2SoapOut) -> Self {
            WsCustomerEnquiryV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateCardholderDetailsSoapIn {
        #[yaserde(rename = "Ws_Update_Cardholder_Details", default)]
        pub body: ports::WsUpdateCardholderDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateCardholderDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateCardholderDetailsSoapIn,
    }

    impl WsUpdateCardholderDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWsUpdateCardholderDetailsSoapIn) -> Self {
            WsUpdateCardholderDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateCardholderDetailsSoapOut {
        #[yaserde(rename = "Ws_UpdateCardholderDetailsResponse", default)]
        pub body: ports::WsUpdateCardholderDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateCardholderDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateCardholderDetailsSoapOut,
    }

    impl WsUpdateCardholderDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUpdateCardholderDetailsSoapOut) -> Self {
            WsUpdateCardholderDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUnLoadStatusChangeSoapIn {
        #[yaserde(rename = "Ws_UnLoad_StatusChange", default)]
        pub body: ports::WsUnLoadStatusChangeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUnLoadStatusChangeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUnLoadStatusChangeSoapIn,
    }

    impl WsUnLoadStatusChangeSoapInSoapEnvelope {
        pub fn new(body: SoapWsUnLoadStatusChangeSoapIn) -> Self {
            WsUnLoadStatusChangeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUnLoadStatusChangeSoapOut {
        #[yaserde(rename = "Ws_UnLoadStatusChangeResponse", default)]
        pub body: ports::WsUnLoadStatusChangeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUnLoadStatusChangeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUnLoadStatusChangeSoapOut,
    }

    impl WsUnLoadStatusChangeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUnLoadStatusChangeSoapOut) -> Self {
            WsUnLoadStatusChangeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsActivateLoadSoapIn {
        #[yaserde(rename = "Ws_Activate_Load", default)]
        pub body: ports::WsActivateLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsActivateLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsActivateLoadSoapIn,
    }

    impl WsActivateLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsActivateLoadSoapIn) -> Self {
            WsActivateLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsActivateLoadSoapOut {
        #[yaserde(rename = "Ws_ActivateLoadResponse", default)]
        pub body: ports::WsActivateLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsActivateLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsActivateLoadSoapOut,
    }

    impl WsActivateLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsActivateLoadSoapOut) -> Self {
            WsActivateLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceAdjustmentSoapIn {
        #[yaserde(rename = "Ws_BalanceAdjustment", default)]
        pub body: ports::WsBalanceAdjustmentSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceAdjustmentSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceAdjustmentSoapIn,
    }

    impl WsBalanceAdjustmentSoapInSoapEnvelope {
        pub fn new(body: SoapWsBalanceAdjustmentSoapIn) -> Self {
            WsBalanceAdjustmentSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBalanceAdjustmentSoapOut {
        #[yaserde(rename = "Ws_BalanceAdjustmentResponse", default)]
        pub body: ports::WsBalanceAdjustmentSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBalanceAdjustmentSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBalanceAdjustmentSoapOut,
    }

    impl WsBalanceAdjustmentSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBalanceAdjustmentSoapOut) -> Self {
            WsBalanceAdjustmentSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsExtendExpirySoapIn {
        #[yaserde(rename = "Ws_ExtendExpiry", default)]
        pub body: ports::WsExtendExpirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsExtendExpirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsExtendExpirySoapIn,
    }

    impl WsExtendExpirySoapInSoapEnvelope {
        pub fn new(body: SoapWsExtendExpirySoapIn) -> Self {
            WsExtendExpirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsExtendExpirySoapOut {
        #[yaserde(rename = "Ws_ExtendExpiryResponse", default)]
        pub body: ports::WsExtendExpirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsExtendExpirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsExtendExpirySoapOut,
    }

    impl WsExtendExpirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsExtendExpirySoapOut) -> Self {
            WsExtendExpirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsTransactionVoidSoapIn {
        #[yaserde(rename = "Ws_Transaction_Void", default)]
        pub body: ports::WsTransactionVoidSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsTransactionVoidSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsTransactionVoidSoapIn,
    }

    impl WsTransactionVoidSoapInSoapEnvelope {
        pub fn new(body: SoapWsTransactionVoidSoapIn) -> Self {
            WsTransactionVoidSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsTransactionVoidSoapOut {
        #[yaserde(rename = "Ws_TransactionVoidResponse", default)]
        pub body: ports::WsTransactionVoidSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsTransactionVoidSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsTransactionVoidSoapOut,
    }

    impl WsTransactionVoidSoapOutSoapEnvelope {
        pub fn new(body: SoapWsTransactionVoidSoapOut) -> Self {
            WsTransactionVoidSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardHolderDetailsEnquirySoapIn {
        #[yaserde(rename = "Ws_CardHolder_Details_Enquiry", default)]
        pub body: ports::WsCardHolderDetailsEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardHolderDetailsEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardHolderDetailsEnquirySoapIn,
    }

    impl WsCardHolderDetailsEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsCardHolderDetailsEnquirySoapIn) -> Self {
            WsCardHolderDetailsEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardHolderDetailsEnquirySoapOut {
        #[yaserde(rename = "Ws_CardHolderDetailsEnquiryResponse", default)]
        pub body: ports::WsCardHolderDetailsEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardHolderDetailsEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardHolderDetailsEnquirySoapOut,
    }

    impl WsCardHolderDetailsEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardHolderDetailsEnquirySoapOut) -> Self {
            WsCardHolderDetailsEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardHolderDetailsEnquiryV2SoapIn {
        #[yaserde(rename = "Ws_CardHolder_Details_Enquiry_V2", default)]
        pub body: ports::WsCardHolderDetailsEnquiryV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardHolderDetailsEnquiryV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardHolderDetailsEnquiryV2SoapIn,
    }

    impl WsCardHolderDetailsEnquiryV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsCardHolderDetailsEnquiryV2SoapIn) -> Self {
            WsCardHolderDetailsEnquiryV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardHolderDetailsEnquiryV2SoapOut {
        #[yaserde(rename = "Ws_CardHolderDetailsEnquiryV2Response", default)]
        pub body: ports::WsCardHolderDetailsEnquiryV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardHolderDetailsEnquiryV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardHolderDetailsEnquiryV2SoapOut,
    }

    impl WsCardHolderDetailsEnquiryV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardHolderDetailsEnquiryV2SoapOut) -> Self {
            WsCardHolderDetailsEnquiryV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPhoneActivationSoapIn {
        #[yaserde(rename = "Ws_Phone_Activation", default)]
        pub body: ports::WsPhoneActivationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPhoneActivationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPhoneActivationSoapIn,
    }

    impl WsPhoneActivationSoapInSoapEnvelope {
        pub fn new(body: SoapWsPhoneActivationSoapIn) -> Self {
            WsPhoneActivationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPhoneActivationSoapOut {
        #[yaserde(rename = "Ws_PhoneActivationResponse", default)]
        pub body: ports::WsPhoneActivationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPhoneActivationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPhoneActivationSoapOut,
    }

    impl WsPhoneActivationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsPhoneActivationSoapOut) -> Self {
            WsPhoneActivationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBulkCreationSoapIn {
        #[yaserde(rename = "Ws_BulkCreation", default)]
        pub body: ports::WsBulkCreationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBulkCreationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBulkCreationSoapIn,
    }

    impl WsBulkCreationSoapInSoapEnvelope {
        pub fn new(body: SoapWsBulkCreationSoapIn) -> Self {
            WsBulkCreationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBulkCreationSoapOut {
        #[yaserde(rename = "Ws_BulkCreationResponse", default)]
        pub body: ports::WsBulkCreationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBulkCreationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBulkCreationSoapOut,
    }

    impl WsBulkCreationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBulkCreationSoapOut) -> Self {
            WsBulkCreationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBulkWalletCreationSoapIn {
        #[yaserde(rename = "Ws_BulkWalletCreation", default)]
        pub body: ports::WsBulkWalletCreationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBulkWalletCreationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBulkWalletCreationSoapIn,
    }

    impl WsBulkWalletCreationSoapInSoapEnvelope {
        pub fn new(body: SoapWsBulkWalletCreationSoapIn) -> Self {
            WsBulkWalletCreationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBulkWalletCreationSoapOut {
        #[yaserde(rename = "Ws_BulkWalletCreationResponse", default)]
        pub body: ports::WsBulkWalletCreationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBulkWalletCreationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBulkWalletCreationSoapOut,
    }

    impl WsBulkWalletCreationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBulkWalletCreationSoapOut) -> Self {
            WsBulkWalletCreationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsWebServiceResultSoapIn {
        #[yaserde(rename = "Ws_WebServiceResult", default)]
        pub body: ports::WsWebServiceResultSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsWebServiceResultSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsWebServiceResultSoapIn,
    }

    impl WsWebServiceResultSoapInSoapEnvelope {
        pub fn new(body: SoapWsWebServiceResultSoapIn) -> Self {
            WsWebServiceResultSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsWebServiceResultSoapOut {
        #[yaserde(rename = "Ws_WebServiceResultResponse", default)]
        pub body: ports::WsWebServiceResultSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsWebServiceResultSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsWebServiceResultSoapOut,
    }

    impl WsWebServiceResultSoapOutSoapEnvelope {
        pub fn new(body: SoapWsWebServiceResultSoapOut) -> Self {
            WsWebServiceResultSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGenericFeesSoapIn {
        #[yaserde(rename = "Ws_Generic_Fees", default)]
        pub body: ports::WsGenericFeesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGenericFeesSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGenericFeesSoapIn,
    }

    impl WsGenericFeesSoapInSoapEnvelope {
        pub fn new(body: SoapWsGenericFeesSoapIn) -> Self {
            WsGenericFeesSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGenericFeesSoapOut {
        #[yaserde(rename = "Ws_GenericFeesResponse", default)]
        pub body: ports::WsGenericFeesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGenericFeesSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGenericFeesSoapOut,
    }

    impl WsGenericFeesSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGenericFeesSoapOut) -> Self {
            WsGenericFeesSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardBalEnqSoapIn {
        #[yaserde(rename = "Ws_Card_BalEnq", default)]
        pub body: ports::WsCardBalEnqSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardBalEnqSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardBalEnqSoapIn,
    }

    impl WsCardBalEnqSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardBalEnqSoapIn) -> Self {
            WsCardBalEnqSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardBalEnqSoapOut {
        #[yaserde(rename = "Ws_CardBalEnqResponse", default)]
        pub body: ports::WsCardBalEnqSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardBalEnqSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardBalEnqSoapOut,
    }

    impl WsCardBalEnqSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardBalEnqSoapOut) -> Self {
            WsCardBalEnqSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPinControlSoapIn {
        #[yaserde(rename = "WS_PinControl", default)]
        pub body: ports::WsPinControlSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPinControlSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPinControlSoapIn,
    }

    impl WsPinControlSoapInSoapEnvelope {
        pub fn new(body: SoapWsPinControlSoapIn) -> Self {
            WsPinControlSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPinControlSoapOut {
        #[yaserde(rename = "Ws_PinControlResponse", default)]
        pub body: ports::WsPinControlSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPinControlSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPinControlSoapOut,
    }

    impl WsPinControlSoapOutSoapEnvelope {
        pub fn new(body: SoapWsPinControlSoapOut) -> Self {
            WsPinControlSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCreateCardSoapIn {
        #[yaserde(rename = "Ws_CreateCard", default)]
        pub body: ports::WsCreateCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCreateCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCreateCardSoapIn,
    }

    impl WsCreateCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsCreateCardSoapIn) -> Self {
            WsCreateCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCreateCardSoapOut {
        #[yaserde(rename = "Ws_CreateCardResponse", default)]
        pub body: ports::WsCreateCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCreateCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCreateCardSoapOut,
    }

    impl WsCreateCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCreateCardSoapOut) -> Self {
            WsCreateCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCreateWalletSoapIn {
        #[yaserde(rename = "Ws_CreateWallet", default)]
        pub body: ports::WsCreateWalletSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCreateWalletSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCreateWalletSoapIn,
    }

    impl WsCreateWalletSoapInSoapEnvelope {
        pub fn new(body: SoapWsCreateWalletSoapIn) -> Self {
            WsCreateWalletSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCreateWalletSoapOut {
        #[yaserde(rename = "Ws_CreateWalletResponse", default)]
        pub body: ports::WsCreateWalletSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCreateWalletSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCreateWalletSoapOut,
    }

    impl WsCreateWalletSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCreateWalletSoapOut) -> Self {
            WsCreateWalletSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsRegenerateSoapIn {
        #[yaserde(rename = "Ws_Regenerate", default)]
        pub body: ports::WsRegenerateSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsRegenerateSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsRegenerateSoapIn,
    }

    impl WsRegenerateSoapInSoapEnvelope {
        pub fn new(body: SoapWsRegenerateSoapIn) -> Self {
            WsRegenerateSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsRegenerateSoapOut {
        #[yaserde(rename = "Ws_RegenerateResponse", default)]
        pub body: ports::WsRegenerateSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsRegenerateSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsRegenerateSoapOut,
    }

    impl WsRegenerateSoapOutSoapEnvelope {
        pub fn new(body: SoapWsRegenerateSoapOut) -> Self {
            WsRegenerateSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsConvertCardSoapIn {
        #[yaserde(rename = "Ws_Convert_Card", default)]
        pub body: ports::WsConvertCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsConvertCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsConvertCardSoapIn,
    }

    impl WsConvertCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsConvertCardSoapIn) -> Self {
            WsConvertCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsConvertCardSoapOut {
        #[yaserde(rename = "Ws_ConvertCardResponse", default)]
        pub body: ports::WsConvertCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsConvertCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsConvertCardSoapOut,
    }

    impl WsConvertCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsConvertCardSoapOut) -> Self {
            WsConvertCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsChangeGroupsSoapIn {
        #[yaserde(rename = "Ws_Change_Groups", default)]
        pub body: ports::WsChangeGroupsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsChangeGroupsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsChangeGroupsSoapIn,
    }

    impl WsChangeGroupsSoapInSoapEnvelope {
        pub fn new(body: SoapWsChangeGroupsSoapIn) -> Self {
            WsChangeGroupsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsChangeGroupsSoapOut {
        #[yaserde(rename = "Ws_ChangeGroupsResponse", default)]
        pub body: ports::WsChangeGroupsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsChangeGroupsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsChangeGroupsSoapOut,
    }

    impl WsChangeGroupsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsChangeGroupsSoapOut) -> Self {
            WsChangeGroupsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCheckSoapIn {
        #[yaserde(rename = "Ws_Check", default)]
        pub body: ports::WsCheckSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCheckSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCheckSoapIn,
    }

    impl WsCheckSoapInSoapEnvelope {
        pub fn new(body: SoapWsCheckSoapIn) -> Self {
            WsCheckSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCheckSoapOut {
        #[yaserde(rename = "Ws_CheckResponse", default)]
        pub body: ports::WsCheckSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCheckSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCheckSoapOut,
    }

    impl WsCheckSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCheckSoapOut) -> Self {
            WsCheckSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSimpleCheckSoapIn {
        #[yaserde(rename = "Ws_Simple_Check", default)]
        pub body: ports::WsSimpleCheckSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSimpleCheckSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSimpleCheckSoapIn,
    }

    impl WsSimpleCheckSoapInSoapEnvelope {
        pub fn new(body: SoapWsSimpleCheckSoapIn) -> Self {
            WsSimpleCheckSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSimpleCheckSoapOut {
        #[yaserde(rename = "Ws_SimpleCheckResponse", default)]
        pub body: ports::WsSimpleCheckSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSimpleCheckSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSimpleCheckSoapOut,
    }

    impl WsSimpleCheckSoapOutSoapEnvelope {
        pub fn new(body: SoapWsSimpleCheckSoapOut) -> Self {
            WsSimpleCheckSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsClientFxSoapIn {
        #[yaserde(rename = "Ws_Client_Fx", default)]
        pub body: ports::WsClientFxSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsClientFxSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsClientFxSoapIn,
    }

    impl WsClientFxSoapInSoapEnvelope {
        pub fn new(body: SoapWsClientFxSoapIn) -> Self {
            WsClientFxSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsClientFxSoapOut {
        #[yaserde(rename = "Ws_ClientFxResponse", default)]
        pub body: ports::WsClientFxSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsClientFxSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsClientFxSoapOut,
    }

    impl WsClientFxSoapOutSoapEnvelope {
        pub fn new(body: SoapWsClientFxSoapOut) -> Self {
            WsClientFxSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsLinkCardsSoapIn {
        #[yaserde(rename = "Ws_Link_Cards", default)]
        pub body: ports::WsLinkCardsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsLinkCardsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsLinkCardsSoapIn,
    }

    impl WsLinkCardsSoapInSoapEnvelope {
        pub fn new(body: SoapWsLinkCardsSoapIn) -> Self {
            WsLinkCardsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsLinkCardsSoapOut {
        #[yaserde(rename = "Ws_LinkCardsResponse", default)]
        pub body: ports::WsLinkCardsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsLinkCardsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsLinkCardsSoapOut,
    }

    impl WsLinkCardsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsLinkCardsSoapOut) -> Self {
            WsLinkCardsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListGroupSoapIn {
        #[yaserde(rename = "Ws_List_Group", default)]
        pub body: ports::WsListGroupSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListGroupSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListGroupSoapIn,
    }

    impl WsListGroupSoapInSoapEnvelope {
        pub fn new(body: SoapWsListGroupSoapIn) -> Self {
            WsListGroupSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListGroupSoapOut {
        #[yaserde(rename = "Ws_ListGroupResponse", default)]
        pub body: ports::WsListGroupSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListGroupSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListGroupSoapOut,
    }

    impl WsListGroupSoapOutSoapEnvelope {
        pub fn new(body: SoapWsListGroupSoapOut) -> Self {
            WsListGroupSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListProductsSoapIn {
        #[yaserde(rename = "Ws_List_Products", default)]
        pub body: ports::WsListProductsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListProductsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListProductsSoapIn,
    }

    impl WsListProductsSoapInSoapEnvelope {
        pub fn new(body: SoapWsListProductsSoapIn) -> Self {
            WsListProductsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListProductsSoapOut {
        #[yaserde(rename = "Ws_ListProductsResponse", default)]
        pub body: ports::WsListProductsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListProductsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListProductsSoapOut,
    }

    impl WsListProductsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsListProductsSoapOut) -> Self {
            WsListProductsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetCardRequestSoapIn {
        #[yaserde(rename = "Ws_GetCardRequest", default)]
        pub body: ports::WsGetCardRequestSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetCardRequestSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetCardRequestSoapIn,
    }

    impl WsGetCardRequestSoapInSoapEnvelope {
        pub fn new(body: SoapWsGetCardRequestSoapIn) -> Self {
            WsGetCardRequestSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetCardRequestSoapOut {
        #[yaserde(rename = "Ws_GetCardRequestResponse", default)]
        pub body: ports::WsGetCardRequestSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetCardRequestSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetCardRequestSoapOut,
    }

    impl WsGetCardRequestSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGetCardRequestSoapOut) -> Self {
            WsGetCardRequestSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetCardRequestStatusSoapIn {
        #[yaserde(rename = "Ws_GetCardRequestStatus", default)]
        pub body: ports::WsGetCardRequestStatusSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetCardRequestStatusSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetCardRequestStatusSoapIn,
    }

    impl WsGetCardRequestStatusSoapInSoapEnvelope {
        pub fn new(body: SoapWsGetCardRequestStatusSoapIn) -> Self {
            WsGetCardRequestStatusSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetCardRequestStatusSoapOut {
        #[yaserde(rename = "Ws_GetCardRequestStatusResponse", default)]
        pub body: ports::WsGetCardRequestStatusSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetCardRequestStatusSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetCardRequestStatusSoapOut,
    }

    impl WsGetCardRequestStatusSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGetCardRequestStatusSoapOut) -> Self {
            WsGetCardRequestStatusSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardAcceptorWhiteListSoapIn {
        #[yaserde(rename = "Ws_CardAcceptorWhiteList", default)]
        pub body: ports::WsCardAcceptorWhiteListSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardAcceptorWhiteListSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardAcceptorWhiteListSoapIn,
    }

    impl WsCardAcceptorWhiteListSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardAcceptorWhiteListSoapIn) -> Self {
            WsCardAcceptorWhiteListSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardAcceptorWhiteListSoapOut {
        #[yaserde(rename = "Ws_CardAcceptorWhiteListResponse", default)]
        pub body: ports::WsCardAcceptorWhiteListSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardAcceptorWhiteListSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardAcceptorWhiteListSoapOut,
    }

    impl WsCardAcceptorWhiteListSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardAcceptorWhiteListSoapOut) -> Self {
            WsCardAcceptorWhiteListSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardAcceptorBlackListSoapIn {
        #[yaserde(rename = "Ws_CardAcceptorBlackList", default)]
        pub body: ports::WsCardAcceptorBlackListSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardAcceptorBlackListSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardAcceptorBlackListSoapIn,
    }

    impl WsCardAcceptorBlackListSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardAcceptorBlackListSoapIn) -> Self {
            WsCardAcceptorBlackListSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardAcceptorBlackListSoapOut {
        #[yaserde(rename = "Ws_CardAcceptorBlackListResponse", default)]
        pub body: ports::WsCardAcceptorBlackListSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardAcceptorBlackListSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardAcceptorBlackListSoapOut,
    }

    impl WsCardAcceptorBlackListSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardAcceptorBlackListSoapOut) -> Self {
            WsCardAcceptorBlackListSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSendMessageSoapIn {
        #[yaserde(rename = "Ws_SendMessage", default)]
        pub body: ports::WsSendMessageSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSendMessageSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSendMessageSoapIn,
    }

    impl WsSendMessageSoapInSoapEnvelope {
        pub fn new(body: SoapWsSendMessageSoapIn) -> Self {
            WsSendMessageSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSendMessageSoapOut {
        #[yaserde(rename = "Ws_SendMessageResponse", default)]
        pub body: ports::WsSendMessageSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSendMessageSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSendMessageSoapOut,
    }

    impl WsSendMessageSoapOutSoapEnvelope {
        pub fn new(body: SoapWsSendMessageSoapOut) -> Self {
            WsSendMessageSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsMVCLoadSoapIn {
        #[yaserde(rename = "Ws_MVCLoad", default)]
        pub body: ports::WsMVCLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsMVCLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsMVCLoadSoapIn,
    }

    impl WsMVCLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsMVCLoadSoapIn) -> Self {
            WsMVCLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsMVCLoadSoapOut {
        #[yaserde(rename = "Ws_MVCLoadResponse", default)]
        pub body: ports::WsMVCLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsMVCLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsMVCLoadSoapOut,
    }

    impl WsMVCLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsMVCLoadSoapOut) -> Self {
            WsMVCLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListPendingFeesSoapIn {
        #[yaserde(rename = "Ws_List_Pending_Fees", default)]
        pub body: ports::WsListPendingFeesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListPendingFeesSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListPendingFeesSoapIn,
    }

    impl WsListPendingFeesSoapInSoapEnvelope {
        pub fn new(body: SoapWsListPendingFeesSoapIn) -> Self {
            WsListPendingFeesSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListPendingFeesSoapOut {
        #[yaserde(rename = "Ws_ListPendingFeesResponse", default)]
        pub body: ports::WsListPendingFeesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListPendingFeesSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListPendingFeesSoapOut,
    }

    impl WsListPendingFeesSoapOutSoapEnvelope {
        pub fn new(body: SoapWsListPendingFeesSoapOut) -> Self {
            WsListPendingFeesSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsWebServiceResultV2SoapIn {
        #[yaserde(rename = "Ws_WebServiceResult_V2", default)]
        pub body: ports::WsWebServiceResultV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsWebServiceResultV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsWebServiceResultV2SoapIn,
    }

    impl WsWebServiceResultV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsWebServiceResultV2SoapIn) -> Self {
            WsWebServiceResultV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsWebServiceResultV2SoapOut {
        #[yaserde(rename = "Ws_WebServiceResult_V2Response", default)]
        pub body: ports::WsWebServiceResultV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsWebServiceResultV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsWebServiceResultV2SoapOut,
    }

    impl WsWebServiceResultV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsWebServiceResultV2SoapOut) -> Self {
            WsWebServiceResultV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetPasscodeSoapIn {
        #[yaserde(rename = "Ws_Get_Passcode", default)]
        pub body: ports::WsGetPasscodeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetPasscodeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetPasscodeSoapIn,
    }

    impl WsGetPasscodeSoapInSoapEnvelope {
        pub fn new(body: SoapWsGetPasscodeSoapIn) -> Self {
            WsGetPasscodeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetPasscodeSoapOut {
        #[yaserde(rename = "Ws_GetPasscodeResponse", default)]
        pub body: ports::WsGetPasscodeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetPasscodeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetPasscodeSoapOut,
    }

    impl WsGetPasscodeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGetPasscodeSoapOut) -> Self {
            WsGetPasscodeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetCardExpireSoonSoapIn {
        #[yaserde(rename = "Ws_Get_Card_ExpireSoon", default)]
        pub body: ports::WsGetCardExpireSoonSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetCardExpireSoonSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetCardExpireSoonSoapIn,
    }

    impl WsGetCardExpireSoonSoapInSoapEnvelope {
        pub fn new(body: SoapWsGetCardExpireSoonSoapIn) -> Self {
            WsGetCardExpireSoonSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGetCardExpireSoonSoapOut {
        #[yaserde(rename = "Ws_GetCardExpireSoonResponse", default)]
        pub body: ports::WsGetCardExpireSoonSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGetCardExpireSoonSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGetCardExpireSoonSoapOut,
    }

    impl WsGetCardExpireSoonSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGetCardExpireSoonSoapOut) -> Self {
            WsGetCardExpireSoonSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSendCardFilesSoapIn {
        #[yaserde(rename = "Ws_Send_CardFiles", default)]
        pub body: ports::WsSendCardFilesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSendCardFilesSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSendCardFilesSoapIn,
    }

    impl WsSendCardFilesSoapInSoapEnvelope {
        pub fn new(body: SoapWsSendCardFilesSoapIn) -> Self {
            WsSendCardFilesSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSendCardFilesSoapOut {
        #[yaserde(rename = "Ws_SendCardFilesResponse", default)]
        pub body: ports::WsSendCardFilesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSendCardFilesSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSendCardFilesSoapOut,
    }

    impl WsSendCardFilesSoapOutSoapEnvelope {
        pub fn new(body: SoapWsSendCardFilesSoapOut) -> Self {
            WsSendCardFilesSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSafeReportsSoapIn {
        #[yaserde(rename = "Ws_SafeReports", default)]
        pub body: ports::WsSafeReportsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSafeReportsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSafeReportsSoapIn,
    }

    impl WsSafeReportsSoapInSoapEnvelope {
        pub fn new(body: SoapWsSafeReportsSoapIn) -> Self {
            WsSafeReportsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSafeReportsSoapOut {
        #[yaserde(rename = "Ws_SafeReportsResponse", default)]
        pub body: ports::WsSafeReportsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSafeReportsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSafeReportsSoapOut,
    }

    impl WsSafeReportsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsSafeReportsSoapOut) -> Self {
            WsSafeReportsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsRegenerateWalletSoapIn {
        #[yaserde(rename = "Ws_RegenerateWallet", default)]
        pub body: ports::WsRegenerateWalletSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsRegenerateWalletSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsRegenerateWalletSoapIn,
    }

    impl WsRegenerateWalletSoapInSoapEnvelope {
        pub fn new(body: SoapWsRegenerateWalletSoapIn) -> Self {
            WsRegenerateWalletSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsRegenerateWalletSoapOut {
        #[yaserde(rename = "Ws_RegenerateWalletResponse", default)]
        pub body: ports::WsRegenerateWalletSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsRegenerateWalletSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsRegenerateWalletSoapOut,
    }

    impl WsRegenerateWalletSoapOutSoapEnvelope {
        pub fn new(body: SoapWsRegenerateWalletSoapOut) -> Self {
            WsRegenerateWalletSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateLoadSourceSoapIn {
        #[yaserde(rename = "Ws_UpdateLoadSource", default)]
        pub body: ports::WsUpdateLoadSourceSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateLoadSourceSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateLoadSourceSoapIn,
    }

    impl WsUpdateLoadSourceSoapInSoapEnvelope {
        pub fn new(body: SoapWsUpdateLoadSourceSoapIn) -> Self {
            WsUpdateLoadSourceSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateLoadSourceSoapOut {
        #[yaserde(rename = "Ws_UpdateLoadSourceResponse", default)]
        pub body: ports::WsUpdateLoadSourceSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateLoadSourceSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateLoadSourceSoapOut,
    }

    impl WsUpdateLoadSourceSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUpdateLoadSourceSoapOut) -> Self {
            WsUpdateLoadSourceSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsMVCUnloadSoapIn {
        #[yaserde(rename = "Ws_MVCUnload", default)]
        pub body: ports::WsMVCUnloadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsMVCUnloadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsMVCUnloadSoapIn,
    }

    impl WsMVCUnloadSoapInSoapEnvelope {
        pub fn new(body: SoapWsMVCUnloadSoapIn) -> Self {
            WsMVCUnloadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsMVCUnloadSoapOut {
        #[yaserde(rename = "Ws_MVCUnloadResponse", default)]
        pub body: ports::WsMVCUnloadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsMVCUnloadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsMVCUnloadSoapOut,
    }

    impl WsMVCUnloadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsMVCUnloadSoapOut) -> Self {
            WsMVCUnloadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsActivateMVCLoadSoapIn {
        #[yaserde(rename = "Ws_Activate_MVCLoad", default)]
        pub body: ports::WsActivateMVCLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsActivateMVCLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsActivateMVCLoadSoapIn,
    }

    impl WsActivateMVCLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsActivateMVCLoadSoapIn) -> Self {
            WsActivateMVCLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsActivateMVCLoadSoapOut {
        #[yaserde(rename = "Ws_ActivateMVCLoadResponse", default)]
        pub body: ports::WsActivateMVCLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsActivateMVCLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsActivateMVCLoadSoapOut,
    }

    impl WsActivateMVCLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsActivateMVCLoadSoapOut) -> Self {
            WsActivateMVCLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsRenewCardSoapIn {
        #[yaserde(rename = "Ws_Renew_Card", default)]
        pub body: ports::WsRenewCardSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsRenewCardSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsRenewCardSoapIn,
    }

    impl WsRenewCardSoapInSoapEnvelope {
        pub fn new(body: SoapWsRenewCardSoapIn) -> Self {
            WsRenewCardSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsRenewCardSoapOut {
        #[yaserde(rename = "Ws_RenewCardResponse", default)]
        pub body: ports::WsRenewCardSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsRenewCardSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsRenewCardSoapOut,
    }

    impl WsRenewCardSoapOutSoapEnvelope {
        pub fn new(body: SoapWsRenewCardSoapOut) -> Self {
            WsRenewCardSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsResetAccumulatorSoapIn {
        #[yaserde(rename = "Ws_ResetAccumulator", default)]
        pub body: ports::WsResetAccumulatorSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsResetAccumulatorSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsResetAccumulatorSoapIn,
    }

    impl WsResetAccumulatorSoapInSoapEnvelope {
        pub fn new(body: SoapWsResetAccumulatorSoapIn) -> Self {
            WsResetAccumulatorSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsResetAccumulatorSoapOut {
        #[yaserde(rename = "Ws_ResetAccumulatorResponse", default)]
        pub body: ports::WsResetAccumulatorSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsResetAccumulatorSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsResetAccumulatorSoapOut,
    }

    impl WsResetAccumulatorSoapOutSoapEnvelope {
        pub fn new(body: SoapWsResetAccumulatorSoapOut) -> Self {
            WsResetAccumulatorSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsEnrolSoapIn {
        #[yaserde(rename = "Ws_Enrol", default)]
        pub body: ports::WsEnrolSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsEnrolSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsEnrolSoapIn,
    }

    impl WsEnrolSoapInSoapEnvelope {
        pub fn new(body: SoapWsEnrolSoapIn) -> Self {
            WsEnrolSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsEnrolSoapOut {
        #[yaserde(rename = "Ws_EnrolResponse", default)]
        pub body: ports::WsEnrolSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsEnrolSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsEnrolSoapOut,
    }

    impl WsEnrolSoapOutSoapEnvelope {
        pub fn new(body: SoapWsEnrolSoapOut) -> Self {
            WsEnrolSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardActivateSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Activate", default)]
        pub body: ports::WsGiftCardActivateSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardActivateSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardActivateSoapIn,
    }

    impl WsGiftCardActivateSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardActivateSoapIn) -> Self {
            WsGiftCardActivateSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardActivateSoapOut {
        #[yaserde(rename = "Ws_GiftCardActivateResponse", default)]
        pub body: ports::WsGiftCardActivateSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardActivateSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardActivateSoapOut,
    }

    impl WsGiftCardActivateSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardActivateSoapOut) -> Self {
            WsGiftCardActivateSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardLoadSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Load", default)]
        pub body: ports::WsGiftCardLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardLoadSoapIn,
    }

    impl WsGiftCardLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardLoadSoapIn) -> Self {
            WsGiftCardLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardLoadSoapOut {
        #[yaserde(rename = "Ws_GiftCardLoadResponse", default)]
        pub body: ports::WsGiftCardLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardLoadSoapOut,
    }

    impl WsGiftCardLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardLoadSoapOut) -> Self {
            WsGiftCardLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUnLoadSoapIn {
        #[yaserde(rename = "Ws_GiftCard_UnLoad", default)]
        pub body: ports::WsGiftCardUnLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUnLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUnLoadSoapIn,
    }

    impl WsGiftCardUnLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUnLoadSoapIn) -> Self {
            WsGiftCardUnLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUnLoadSoapOut {
        #[yaserde(rename = "Ws_GiftCardUnLoadResponse", default)]
        pub body: ports::WsGiftCardUnLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUnLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUnLoadSoapOut,
    }

    impl WsGiftCardUnLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUnLoadSoapOut) -> Self {
            WsGiftCardUnLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardStatusChangeSoapIn {
        #[yaserde(rename = "Ws_GiftCard_StatusChange", default)]
        pub body: ports::WsGiftCardStatusChangeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardStatusChangeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardStatusChangeSoapIn,
    }

    impl WsGiftCardStatusChangeSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardStatusChangeSoapIn) -> Self {
            WsGiftCardStatusChangeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardStatusChangeSoapOut {
        #[yaserde(rename = "Ws_GiftCardStatusChangeResponse", default)]
        pub body: ports::WsGiftCardStatusChangeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardStatusChangeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardStatusChangeSoapOut,
    }

    impl WsGiftCardStatusChangeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardStatusChangeSoapOut) -> Self {
            WsGiftCardStatusChangeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardEnquirySoapIn {
        #[yaserde(rename = "Ws_GiftCard_Enquiry", default)]
        pub body: ports::WsGiftCardEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardEnquirySoapIn,
    }

    impl WsGiftCardEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardEnquirySoapIn) -> Self {
            WsGiftCardEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardEnquirySoapOut {
        #[yaserde(rename = "Ws_GiftCardEnquiryResponse", default)]
        pub body: ports::WsGiftCardEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardEnquirySoapOut,
    }

    impl WsGiftCardEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardEnquirySoapOut) -> Self {
            WsGiftCardEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBalanceTransferSoapIn {
        #[yaserde(rename = "Ws_GiftCard_BalanceTransfer", default)]
        pub body: ports::WsGiftCardBalanceTransferSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBalanceTransferSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBalanceTransferSoapIn,
    }

    impl WsGiftCardBalanceTransferSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBalanceTransferSoapIn) -> Self {
            WsGiftCardBalanceTransferSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBalanceTransferSoapOut {
        #[yaserde(rename = "Ws_GiftCardBalanceTransferResponse", default)]
        pub body: ports::WsGiftCardBalanceTransferSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBalanceTransferSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBalanceTransferSoapOut,
    }

    impl WsGiftCardBalanceTransferSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBalanceTransferSoapOut) -> Self {
            WsGiftCardBalanceTransferSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBalanceEnquirySoapIn {
        #[yaserde(rename = "Ws_GiftCard_Balance_Enquiry", default)]
        pub body: ports::WsGiftCardBalanceEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBalanceEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBalanceEnquirySoapIn,
    }

    impl WsGiftCardBalanceEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBalanceEnquirySoapIn) -> Self {
            WsGiftCardBalanceEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBalanceEnquirySoapOut {
        #[yaserde(rename = "Ws_GiftCardBalanceEnquiryResponse", default)]
        pub body: ports::WsGiftCardBalanceEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBalanceEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBalanceEnquirySoapOut,
    }

    impl WsGiftCardBalanceEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBalanceEnquirySoapOut) -> Self {
            WsGiftCardBalanceEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardCardStatementSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Card_Statement", default)]
        pub body: ports::WsGiftCardCardStatementSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardCardStatementSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardCardStatementSoapIn,
    }

    impl WsGiftCardCardStatementSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardCardStatementSoapIn) -> Self {
            WsGiftCardCardStatementSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardCardStatementSoapOut {
        #[yaserde(rename = "Ws_GiftCardCardStatementResponse", default)]
        pub body: ports::WsGiftCardCardStatementSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardCardStatementSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardCardStatementSoapOut,
    }

    impl WsGiftCardCardStatementSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardCardStatementSoapOut) -> Self {
            WsGiftCardCardStatementSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUpdateCardholderDetailsSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Update_Cardholder_Details", default)]
        pub body: ports::WsGiftCardUpdateCardholderDetailsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUpdateCardholderDetailsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUpdateCardholderDetailsSoapIn,
    }

    impl WsGiftCardUpdateCardholderDetailsSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUpdateCardholderDetailsSoapIn) -> Self {
            WsGiftCardUpdateCardholderDetailsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUpdateCardholderDetailsSoapOut {
        #[yaserde(rename = "Ws_GiftCardUpdateCardholderDetailsResponse", default)]
        pub body: ports::WsGiftCardUpdateCardholderDetailsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUpdateCardholderDetailsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUpdateCardholderDetailsSoapOut,
    }

    impl WsGiftCardUpdateCardholderDetailsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUpdateCardholderDetailsSoapOut) -> Self {
            WsGiftCardUpdateCardholderDetailsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUnLoadStatusChangeSoapIn {
        #[yaserde(rename = "Ws_GiftCard_UnLoad_StatusChange", default)]
        pub body: ports::WsGiftCardUnLoadStatusChangeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUnLoadStatusChangeSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUnLoadStatusChangeSoapIn,
    }

    impl WsGiftCardUnLoadStatusChangeSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUnLoadStatusChangeSoapIn) -> Self {
            WsGiftCardUnLoadStatusChangeSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUnLoadStatusChangeSoapOut {
        #[yaserde(rename = "Ws_GiftCardUnLoadStatusChangeResponse", default)]
        pub body: ports::WsGiftCardUnLoadStatusChangeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUnLoadStatusChangeSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUnLoadStatusChangeSoapOut,
    }

    impl WsGiftCardUnLoadStatusChangeSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUnLoadStatusChangeSoapOut) -> Self {
            WsGiftCardUnLoadStatusChangeSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardActivateLoadSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Activate_Load", default)]
        pub body: ports::WsGiftCardActivateLoadSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardActivateLoadSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardActivateLoadSoapIn,
    }

    impl WsGiftCardActivateLoadSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardActivateLoadSoapIn) -> Self {
            WsGiftCardActivateLoadSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardActivateLoadSoapOut {
        #[yaserde(rename = "Ws_GiftCardActivateLoadResponse", default)]
        pub body: ports::WsGiftCardActivateLoadSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardActivateLoadSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardActivateLoadSoapOut,
    }

    impl WsGiftCardActivateLoadSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardActivateLoadSoapOut) -> Self {
            WsGiftCardActivateLoadSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBalanceAdjustmentSoapIn {
        #[yaserde(rename = "Ws_GiftCard_BalanceAdjustment", default)]
        pub body: ports::WsGiftCardBalanceAdjustmentSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBalanceAdjustmentSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBalanceAdjustmentSoapIn,
    }

    impl WsGiftCardBalanceAdjustmentSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBalanceAdjustmentSoapIn) -> Self {
            WsGiftCardBalanceAdjustmentSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBalanceAdjustmentSoapOut {
        #[yaserde(rename = "Ws_GiftCardBalanceAdjustmentResponse", default)]
        pub body: ports::WsGiftCardBalanceAdjustmentSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBalanceAdjustmentSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBalanceAdjustmentSoapOut,
    }

    impl WsGiftCardBalanceAdjustmentSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBalanceAdjustmentSoapOut) -> Self {
            WsGiftCardBalanceAdjustmentSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardExtendExpirySoapIn {
        #[yaserde(rename = "Ws_GiftCard_ExtendExpiry", default)]
        pub body: ports::WsGiftCardExtendExpirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardExtendExpirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardExtendExpirySoapIn,
    }

    impl WsGiftCardExtendExpirySoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardExtendExpirySoapIn) -> Self {
            WsGiftCardExtendExpirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardExtendExpirySoapOut {
        #[yaserde(rename = "Ws_GiftCardExtendExpiryResponse", default)]
        pub body: ports::WsGiftCardExtendExpirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardExtendExpirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardExtendExpirySoapOut,
    }

    impl WsGiftCardExtendExpirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardExtendExpirySoapOut) -> Self {
            WsGiftCardExtendExpirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardTransactionVoidSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Transaction_Void", default)]
        pub body: ports::WsGiftCardTransactionVoidSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardTransactionVoidSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardTransactionVoidSoapIn,
    }

    impl WsGiftCardTransactionVoidSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardTransactionVoidSoapIn) -> Self {
            WsGiftCardTransactionVoidSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardTransactionVoidSoapOut {
        #[yaserde(rename = "Ws_GiftCardTransactionVoidResponse", default)]
        pub body: ports::WsGiftCardTransactionVoidSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardTransactionVoidSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardTransactionVoidSoapOut,
    }

    impl WsGiftCardTransactionVoidSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardTransactionVoidSoapOut) -> Self {
            WsGiftCardTransactionVoidSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardCardHolderDetailsEnquirySoapIn {
        #[yaserde(rename = "Ws_GiftCard_CardHolder_Details_Enquiry", default)]
        pub body: ports::WsGiftCardCardHolderDetailsEnquirySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardCardHolderDetailsEnquirySoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardCardHolderDetailsEnquirySoapIn,
    }

    impl WsGiftCardCardHolderDetailsEnquirySoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardCardHolderDetailsEnquirySoapIn) -> Self {
            WsGiftCardCardHolderDetailsEnquirySoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardCardHolderDetailsEnquirySoapOut {
        #[yaserde(rename = "Ws_GiftCardCardHolderDetailsEnquiryResponse", default)]
        pub body: ports::WsGiftCardCardHolderDetailsEnquirySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardCardHolderDetailsEnquirySoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardCardHolderDetailsEnquirySoapOut,
    }

    impl WsGiftCardCardHolderDetailsEnquirySoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardCardHolderDetailsEnquirySoapOut) -> Self {
            WsGiftCardCardHolderDetailsEnquirySoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardPhoneActivationSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Phone_Activation", default)]
        pub body: ports::WsGiftCardPhoneActivationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardPhoneActivationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardPhoneActivationSoapIn,
    }

    impl WsGiftCardPhoneActivationSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardPhoneActivationSoapIn) -> Self {
            WsGiftCardPhoneActivationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardPhoneActivationSoapOut {
        #[yaserde(rename = "Ws_GiftCardPhoneActivationResponse", default)]
        pub body: ports::WsGiftCardPhoneActivationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardPhoneActivationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardPhoneActivationSoapOut,
    }

    impl WsGiftCardPhoneActivationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardPhoneActivationSoapOut) -> Self {
            WsGiftCardPhoneActivationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBulkCreationSoapIn {
        #[yaserde(rename = "Ws_GiftCard_BulkCreation", default)]
        pub body: ports::WsGiftCardBulkCreationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBulkCreationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBulkCreationSoapIn,
    }

    impl WsGiftCardBulkCreationSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBulkCreationSoapIn) -> Self {
            WsGiftCardBulkCreationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardBulkCreationSoapOut {
        #[yaserde(rename = "Ws_GiftCardBulkCreationResponse", default)]
        pub body: ports::WsGiftCardBulkCreationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardBulkCreationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardBulkCreationSoapOut,
    }

    impl WsGiftCardBulkCreationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardBulkCreationSoapOut) -> Self {
            WsGiftCardBulkCreationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardWebServiceResultSoapIn {
        #[yaserde(rename = "Ws_GiftCard_WebServiceResult", default)]
        pub body: ports::WsGiftCardWebServiceResultSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardWebServiceResultSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardWebServiceResultSoapIn,
    }

    impl WsGiftCardWebServiceResultSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardWebServiceResultSoapIn) -> Self {
            WsGiftCardWebServiceResultSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardWebServiceResultSoapOut {
        #[yaserde(rename = "Ws_GiftCardWebServiceResultResponse", default)]
        pub body: ports::WsGiftCardWebServiceResultSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardWebServiceResultSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardWebServiceResultSoapOut,
    }

    impl WsGiftCardWebServiceResultSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardWebServiceResultSoapOut) -> Self {
            WsGiftCardWebServiceResultSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardGenericFeesSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Generic_Fees", default)]
        pub body: ports::WsGiftCardGenericFeesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardGenericFeesSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardGenericFeesSoapIn,
    }

    impl WsGiftCardGenericFeesSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardGenericFeesSoapIn) -> Self {
            WsGiftCardGenericFeesSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardGenericFeesSoapOut {
        #[yaserde(rename = "Ws_GiftCardGenericFeesResponse", default)]
        pub body: ports::WsGiftCardGenericFeesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardGenericFeesSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardGenericFeesSoapOut,
    }

    impl WsGiftCardGenericFeesSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardGenericFeesSoapOut) -> Self {
            WsGiftCardGenericFeesSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardPinControlSoapIn {
        #[yaserde(rename = "Ws_GiftCard_PinControl", default)]
        pub body: ports::WsGiftCardPinControlSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardPinControlSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardPinControlSoapIn,
    }

    impl WsGiftCardPinControlSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardPinControlSoapIn) -> Self {
            WsGiftCardPinControlSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardPinControlSoapOut {
        #[yaserde(rename = "Ws_GiftCardPinControlResponse", default)]
        pub body: ports::WsGiftCardPinControlSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardPinControlSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardPinControlSoapOut,
    }

    impl WsGiftCardPinControlSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardPinControlSoapOut) -> Self {
            WsGiftCardPinControlSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUpdateLoadSourceSoapIn {
        #[yaserde(rename = "Ws_GiftCard_UpdateLoadSource", default)]
        pub body: ports::WsGiftCardUpdateLoadSourceSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUpdateLoadSourceSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUpdateLoadSourceSoapIn,
    }

    impl WsGiftCardUpdateLoadSourceSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUpdateLoadSourceSoapIn) -> Self {
            WsGiftCardUpdateLoadSourceSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardUpdateLoadSourceSoapOut {
        #[yaserde(rename = "Ws_GiftCardUpdateLoadSourceResponse", default)]
        pub body: ports::WsGiftCardUpdateLoadSourceSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardUpdateLoadSourceSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardUpdateLoadSourceSoapOut,
    }

    impl WsGiftCardUpdateLoadSourceSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardUpdateLoadSourceSoapOut) -> Self {
            WsGiftCardUpdateLoadSourceSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardActivateLoadProductTpyeCPSoapIn {
        #[yaserde(rename = "Ws_GiftCard_Activate_Load_ProductTpye_CP", default)]
        pub body: ports::WsGiftCardActivateLoadProductTpyeCPSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardActivateLoadProductTpyeCPSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardActivateLoadProductTpyeCPSoapIn,
    }

    impl WsGiftCardActivateLoadProductTpyeCPSoapInSoapEnvelope {
        pub fn new(body: SoapWsGiftCardActivateLoadProductTpyeCPSoapIn) -> Self {
            WsGiftCardActivateLoadProductTpyeCPSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsGiftCardActivateLoadProductTpyeCPSoapOut {
        #[yaserde(rename = "Ws_GiftCardActivateLoadProductTpyeCPResponse", default)]
        pub body: ports::WsGiftCardActivateLoadProductTpyeCPSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsGiftCardActivateLoadProductTpyeCPSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsGiftCardActivateLoadProductTpyeCPSoapOut,
    }

    impl WsGiftCardActivateLoadProductTpyeCPSoapOutSoapEnvelope {
        pub fn new(body: SoapWsGiftCardActivateLoadProductTpyeCPSoapOut) -> Self {
            WsGiftCardActivateLoadProductTpyeCPSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardTransactionXMLSoapIn {
        #[yaserde(rename = "Ws_Card_TransactionXML", default)]
        pub body: ports::WsCardTransactionXMLSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardTransactionXMLSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardTransactionXMLSoapIn,
    }

    impl WsCardTransactionXMLSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardTransactionXMLSoapIn) -> Self {
            WsCardTransactionXMLSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardTransactionXMLSoapOut {
        #[yaserde(rename = "Ws_CardTransactionXMLResponse", default)]
        pub body: ports::WsCardTransactionXMLSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardTransactionXMLSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardTransactionXMLSoapOut,
    }

    impl WsCardTransactionXMLSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardTransactionXMLSoapOut) -> Self {
            WsCardTransactionXMLSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardChangeGroupsSoapIn {
        #[yaserde(rename = "Ws_Card_Change_Groups", default)]
        pub body: ports::WsCardChangeGroupsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardChangeGroupsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardChangeGroupsSoapIn,
    }

    impl WsCardChangeGroupsSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardChangeGroupsSoapIn) -> Self {
            WsCardChangeGroupsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardChangeGroupsSoapOut {
        #[yaserde(rename = "Ws_CardChangeGroupsResponse", default)]
        pub body: ports::WsCardChangeGroupsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardChangeGroupsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardChangeGroupsSoapOut,
    }

    impl WsCardChangeGroupsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardChangeGroupsSoapOut) -> Self {
            WsCardChangeGroupsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardChangeCardacceptorListSoapIn {
        #[yaserde(rename = "Ws_Card_Change_Cardacceptor_List", default)]
        pub body: ports::WsCardChangeCardacceptorListSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardChangeCardacceptorListSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardChangeCardacceptorListSoapIn,
    }

    impl WsCardChangeCardacceptorListSoapInSoapEnvelope {
        pub fn new(body: SoapWsCardChangeCardacceptorListSoapIn) -> Self {
            WsCardChangeCardacceptorListSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCardChangeCardacceptorListSoapOut {
        #[yaserde(rename = "Ws_CardChangeCardacceptorListResponse", default)]
        pub body: ports::WsCardChangeCardacceptorListSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCardChangeCardacceptorListSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCardChangeCardacceptorListSoapOut,
    }

    impl WsCardChangeCardacceptorListSoapOutSoapEnvelope {
        pub fn new(body: SoapWsCardChangeCardacceptorListSoapOut) -> Self {
            WsCardChangeCardacceptorListSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsChangeCardacceptorListSoapIn {
        #[yaserde(rename = "Ws_Change_Cardacceptor_List", default)]
        pub body: ports::WsChangeCardacceptorListSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsChangeCardacceptorListSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsChangeCardacceptorListSoapIn,
    }

    impl WsChangeCardacceptorListSoapInSoapEnvelope {
        pub fn new(body: SoapWsChangeCardacceptorListSoapIn) -> Self {
            WsChangeCardacceptorListSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsChangeCardacceptorListSoapOut {
        #[yaserde(rename = "Ws_ChangeCardacceptorListResponse", default)]
        pub body: ports::WsChangeCardacceptorListSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsChangeCardacceptorListSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsChangeCardacceptorListSoapOut,
    }

    impl WsChangeCardacceptorListSoapOutSoapEnvelope {
        pub fn new(body: SoapWsChangeCardacceptorListSoapOut) -> Self {
            WsChangeCardacceptorListSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsAddressMatchCheckingSoapIn {
        #[yaserde(rename = "Ws_AddressMatchChecking", default)]
        pub body: ports::WsAddressMatchCheckingSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsAddressMatchCheckingSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsAddressMatchCheckingSoapIn,
    }

    impl WsAddressMatchCheckingSoapInSoapEnvelope {
        pub fn new(body: SoapWsAddressMatchCheckingSoapIn) -> Self {
            WsAddressMatchCheckingSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsAddressMatchCheckingSoapOut {
        #[yaserde(rename = "Ws_AddressMatchCheckingResponse", default)]
        pub body: ports::WsAddressMatchCheckingSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsAddressMatchCheckingSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsAddressMatchCheckingSoapOut,
    }

    impl WsAddressMatchCheckingSoapOutSoapEnvelope {
        pub fn new(body: SoapWsAddressMatchCheckingSoapOut) -> Self {
            WsAddressMatchCheckingSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsLicenseVerificationSoapIn {
        #[yaserde(rename = "Ws_LicenseVerification", default)]
        pub body: ports::WsLicenseVerificationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsLicenseVerificationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsLicenseVerificationSoapIn,
    }

    impl WsLicenseVerificationSoapInSoapEnvelope {
        pub fn new(body: SoapWsLicenseVerificationSoapIn) -> Self {
            WsLicenseVerificationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsLicenseVerificationSoapOut {
        #[yaserde(rename = "Ws_LicenseVerificationResponse", default)]
        pub body: ports::WsLicenseVerificationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsLicenseVerificationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsLicenseVerificationSoapOut,
    }

    impl WsLicenseVerificationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsLicenseVerificationSoapOut) -> Self {
            WsLicenseVerificationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPassportVerificationSoapIn {
        #[yaserde(rename = "Ws_PassportVerification", default)]
        pub body: ports::WsPassportVerificationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPassportVerificationSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPassportVerificationSoapIn,
    }

    impl WsPassportVerificationSoapInSoapEnvelope {
        pub fn new(body: SoapWsPassportVerificationSoapIn) -> Self {
            WsPassportVerificationSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsPassportVerificationSoapOut {
        #[yaserde(rename = "Ws_PassportVerificationResponse", default)]
        pub body: ports::WsPassportVerificationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsPassportVerificationSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsPassportVerificationSoapOut,
    }

    impl WsPassportVerificationSoapOutSoapEnvelope {
        pub fn new(body: SoapWsPassportVerificationSoapOut) -> Self {
            WsPassportVerificationSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSanctionsPEPCheckSoapIn {
        #[yaserde(rename = "Ws_Sanctions_PEP_Check", default)]
        pub body: ports::WsSanctionsPEPCheckSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSanctionsPEPCheckSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSanctionsPEPCheckSoapIn,
    }

    impl WsSanctionsPEPCheckSoapInSoapEnvelope {
        pub fn new(body: SoapWsSanctionsPEPCheckSoapIn) -> Self {
            WsSanctionsPEPCheckSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSanctionsPEPCheckSoapOut {
        #[yaserde(rename = "Ws_SanctionsPEPCheckResponse", default)]
        pub body: ports::WsSanctionsPEPCheckSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSanctionsPEPCheckSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSanctionsPEPCheckSoapOut,
    }

    impl WsSanctionsPEPCheckSoapOutSoapEnvelope {
        pub fn new(body: SoapWsSanctionsPEPCheckSoapOut) -> Self {
            WsSanctionsPEPCheckSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSanctionsPEPCheckV2SoapIn {
        #[yaserde(rename = "Ws_Sanctions_PEP_Check_V2", default)]
        pub body: ports::WsSanctionsPEPCheckV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSanctionsPEPCheckV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSanctionsPEPCheckV2SoapIn,
    }

    impl WsSanctionsPEPCheckV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsSanctionsPEPCheckV2SoapIn) -> Self {
            WsSanctionsPEPCheckV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsSanctionsPEPCheckV2SoapOut {
        #[yaserde(rename = "Ws_SanctionsPEPCheckV2Response", default)]
        pub body: ports::WsSanctionsPEPCheckV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsSanctionsPEPCheckV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsSanctionsPEPCheckV2SoapOut,
    }

    impl WsSanctionsPEPCheckV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsSanctionsPEPCheckV2SoapOut) -> Self {
            WsSanctionsPEPCheckV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListSanctionsPEPSoapIn {
        #[yaserde(rename = "Ws_List_Sanctions_PEP", default)]
        pub body: ports::WsListSanctionsPEPSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListSanctionsPEPSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListSanctionsPEPSoapIn,
    }

    impl WsListSanctionsPEPSoapInSoapEnvelope {
        pub fn new(body: SoapWsListSanctionsPEPSoapIn) -> Self {
            WsListSanctionsPEPSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListSanctionsPEPSoapOut {
        #[yaserde(rename = "Ws_ListSanctionsPEPResponse", default)]
        pub body: ports::WsListSanctionsPEPSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListSanctionsPEPSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListSanctionsPEPSoapOut,
    }

    impl WsListSanctionsPEPSoapOutSoapEnvelope {
        pub fn new(body: SoapWsListSanctionsPEPSoapOut) -> Self {
            WsListSanctionsPEPSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListSanctionsPEPMatchesSoapIn {
        #[yaserde(rename = "Ws_List_Sanctions_PEP_Matches", default)]
        pub body: ports::WsListSanctionsPEPMatchesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListSanctionsPEPMatchesSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListSanctionsPEPMatchesSoapIn,
    }

    impl WsListSanctionsPEPMatchesSoapInSoapEnvelope {
        pub fn new(body: SoapWsListSanctionsPEPMatchesSoapIn) -> Self {
            WsListSanctionsPEPMatchesSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsListSanctionsPEPMatchesSoapOut {
        #[yaserde(rename = "Ws_ListSanctionsPEPMatchesResponse", default)]
        pub body: ports::WsListSanctionsPEPMatchesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsListSanctionsPEPMatchesSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsListSanctionsPEPMatchesSoapOut,
    }

    impl WsListSanctionsPEPMatchesSoapOutSoapEnvelope {
        pub fn new(body: SoapWsListSanctionsPEPMatchesSoapOut) -> Self {
            WsListSanctionsPEPMatchesSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateSanctionsPEPMatchesSoapIn {
        #[yaserde(rename = "Ws_Update_Sanctions_PEP_Matches", default)]
        pub body: ports::WsUpdateSanctionsPEPMatchesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateSanctionsPEPMatchesSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateSanctionsPEPMatchesSoapIn,
    }

    impl WsUpdateSanctionsPEPMatchesSoapInSoapEnvelope {
        pub fn new(body: SoapWsUpdateSanctionsPEPMatchesSoapIn) -> Self {
            WsUpdateSanctionsPEPMatchesSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsUpdateSanctionsPEPMatchesSoapOut {
        #[yaserde(rename = "Ws_UpdateSanctionsPEPMatchesResponse", default)]
        pub body: ports::WsUpdateSanctionsPEPMatchesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsUpdateSanctionsPEPMatchesSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsUpdateSanctionsPEPMatchesSoapOut,
    }

    impl WsUpdateSanctionsPEPMatchesSoapOutSoapEnvelope {
        pub fn new(body: SoapWsUpdateSanctionsPEPMatchesSoapOut) -> Self {
            WsUpdateSanctionsPEPMatchesSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCreateCardV2SoapIn {
        #[yaserde(rename = "Ws_CreateCard_V2", default)]
        pub body: ports::WsCreateCardV2SoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCreateCardV2SoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCreateCardV2SoapIn,
    }

    impl WsCreateCardV2SoapInSoapEnvelope {
        pub fn new(body: SoapWsCreateCardV2SoapIn) -> Self {
            WsCreateCardV2SoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsCreateCardV2SoapOut {
        #[yaserde(rename = "Ws_CreateCardV2Response", default)]
        pub body: ports::WsCreateCardV2SoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsCreateCardV2SoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsCreateCardV2SoapOut,
    }

    impl WsCreateCardV2SoapOutSoapEnvelope {
        pub fn new(body: SoapWsCreateCardV2SoapOut) -> Self {
            WsCreateCardV2SoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingReturnBankDetailsFromTokenSoapIn {
        #[yaserde(rename = "Ws_Banking_ReturnBankDetailsFromToken", default)]
        pub body: ports::WsBankingReturnBankDetailsFromTokenSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingReturnBankDetailsFromTokenSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingReturnBankDetailsFromTokenSoapIn,
    }

    impl WsBankingReturnBankDetailsFromTokenSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingReturnBankDetailsFromTokenSoapIn) -> Self {
            WsBankingReturnBankDetailsFromTokenSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingReturnBankDetailsFromTokenSoapOut {
        #[yaserde(rename = "Ws_BankingReturnBankDetailsFromTokenResponse", default)]
        pub body: ports::WsBankingReturnBankDetailsFromTokenSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingReturnBankDetailsFromTokenSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingReturnBankDetailsFromTokenSoapOut,
    }

    impl WsBankingReturnBankDetailsFromTokenSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingReturnBankDetailsFromTokenSoapOut) -> Self {
            WsBankingReturnBankDetailsFromTokenSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingChangeAccountBankingFeaturesStatusSoapIn {
        #[yaserde(rename = "Ws_Banking_ChangeAccountBankingFeaturesStatus", default)]
        pub body: ports::WsBankingChangeAccountBankingFeaturesStatusSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingChangeAccountBankingFeaturesStatusSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingChangeAccountBankingFeaturesStatusSoapIn,
    }

    impl WsBankingChangeAccountBankingFeaturesStatusSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingChangeAccountBankingFeaturesStatusSoapIn) -> Self {
            WsBankingChangeAccountBankingFeaturesStatusSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingChangeAccountBankingFeaturesStatusSoapOut {
        #[yaserde(
            rename = "Ws_BankingChangeAccountBankingFeaturesStatusResponse",
            default
        )]
        pub body: ports::WsBankingChangeAccountBankingFeaturesStatusSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingChangeAccountBankingFeaturesStatusSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingChangeAccountBankingFeaturesStatusSoapOut,
    }

    impl WsBankingChangeAccountBankingFeaturesStatusSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingChangeAccountBankingFeaturesStatusSoapOut) -> Self {
            WsBankingChangeAccountBankingFeaturesStatusSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingTransferFundsSoapIn {
        #[yaserde(rename = "Ws_Banking_TransferFunds", default)]
        pub body: ports::WsBankingTransferFundsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingTransferFundsSoapInSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingTransferFundsSoapIn,
    }

    impl WsBankingTransferFundsSoapInSoapEnvelope {
        pub fn new(body: SoapWsBankingTransferFundsSoapIn) -> Self {
            WsBankingTransferFundsSoapInSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapWsBankingTransferFundsSoapOut {
        #[yaserde(rename = "Ws_BankingTransferFundsResponse", default)]
        pub body: ports::WsBankingTransferFundsSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "xsd: http://www.w3.org/2001/XMLSchema",
        prefix = "soap"
    )]
    pub struct WsBankingTransferFundsSoapOutSoapEnvelope {
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soap")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soap")]
        pub body: SoapWsBankingTransferFundsSoapOut,
    }

    impl WsBankingTransferFundsSoapOutSoapEnvelope {
        pub fn new(body: SoapWsBankingTransferFundsSoapOut) -> Self {
            WsBankingTransferFundsSoapOutSoapEnvelope {
                tnsattr: Option::Some("http://microsoft.com/wsdl/types/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }
}
