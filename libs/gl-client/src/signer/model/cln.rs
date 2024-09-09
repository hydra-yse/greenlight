//
// This file was generated by `gengrpc` from the CLN JSON-Schema.
// Do not edit this file.
//

use anyhow::anyhow;
use prost::Message;
pub use crate::pb::cln::*;
use super::Request;

pub fn decode_request(uri: &str, p: &[u8]) -> anyhow::Result<Request> {
    Ok(match uri {
	"/cln.Node/Getinfo" => Request::Getinfo(GetinfoRequest::decode(p)?),
	"/cln.Node/ListPeers" => Request::ListPeers(ListpeersRequest::decode(p)?),
	"/cln.Node/ListFunds" => Request::ListFunds(ListfundsRequest::decode(p)?),
	"/cln.Node/SendPay" => Request::SendPay(SendpayRequest::decode(p)?),
	"/cln.Node/ListChannels" => Request::ListChannels(ListchannelsRequest::decode(p)?),
	"/cln.Node/AddGossip" => Request::AddGossip(AddgossipRequest::decode(p)?),
	"/cln.Node/AutoCleanInvoice" => Request::AutoCleanInvoice(AutocleaninvoiceRequest::decode(p)?),
	"/cln.Node/CheckMessage" => Request::CheckMessage(CheckmessageRequest::decode(p)?),
	"/cln.Node/Close" => Request::Close(CloseRequest::decode(p)?),
	"/cln.Node/ConnectPeer" => Request::Connect(ConnectRequest::decode(p)?),
	"/cln.Node/CreateInvoice" => Request::CreateInvoice(CreateinvoiceRequest::decode(p)?),
	"/cln.Node/Datastore" => Request::Datastore(DatastoreRequest::decode(p)?),
	"/cln.Node/CreateOnion" => Request::CreateOnion(CreateonionRequest::decode(p)?),
	"/cln.Node/DelDatastore" => Request::DelDatastore(DeldatastoreRequest::decode(p)?),
	"/cln.Node/DelExpiredInvoice" => Request::DelExpiredInvoice(DelinvoiceRequest::decode(p)?),
	"/cln.Node/DelInvoice" => Request::DelInvoice(DelinvoiceRequest::decode(p)?),
	"/cln.Node/Invoice" => Request::Invoice(InvoiceRequest::decode(p)?),
	"/cln.Node/ListDatastore" => Request::ListDatastore(ListdatastoreRequest::decode(p)?),
	"/cln.Node/ListInvoices" => Request::ListInvoices(ListinvoicesRequest::decode(p)?),
	"/cln.Node/SendOnion" => Request::SendOnion(SendonionRequest::decode(p)?),
	"/cln.Node/ListSendPays" => Request::ListSendPays(ListsendpaysRequest::decode(p)?),
	"/cln.Node/ListTransactions" => Request::ListTransactions(ListtransactionsRequest::decode(p)?),
	"/cln.Node/Pay" => Request::Pay(PayRequest::decode(p)?),
	"/cln.Node/ListNodes" => Request::ListNodes(ListnodesRequest::decode(p)?),
	"/cln.Node/WaitAnyInvoice" => Request::WaitAnyInvoice(WaitanyinvoiceRequest::decode(p)?),
	"/cln.Node/WaitInvoice" => Request::WaitInvoice(WaitinvoiceRequest::decode(p)?),
	"/cln.Node/WaitSendPay" => Request::WaitSendPay(WaitsendpayRequest::decode(p)?),
	"/cln.Node/NewAddr" => Request::NewAddr(NewaddrRequest::decode(p)?),
	"/cln.Node/Withdraw" => Request::Withdraw(WithdrawRequest::decode(p)?),
	"/cln.Node/KeySend" => Request::KeySend(KeysendRequest::decode(p)?),
	"/cln.Node/FundPsbt" => Request::FundPsbt(FundpsbtRequest::decode(p)?),
	"/cln.Node/SendPsbt" => Request::SendPsbt(SendpsbtRequest::decode(p)?),
	"/cln.Node/SignPsbt" => Request::SignPsbt(SignpsbtRequest::decode(p)?),
	"/cln.Node/UtxoPsbt" => Request::UtxoPsbt(UtxopsbtRequest::decode(p)?),
	"/cln.Node/TxDiscard" => Request::TxDiscard(TxdiscardRequest::decode(p)?),
	"/cln.Node/TxPrepare" => Request::TxPrepare(TxprepareRequest::decode(p)?),
	"/cln.Node/TxSend" => Request::TxSend(TxsendRequest::decode(p)?),
	"/cln.Node/Disconnect" => Request::Disconnect(DisconnectRequest::decode(p)?),
	"/cln.Node/Feerates" => Request::Feerates(FeeratesRequest::decode(p)?),
	"/cln.Node/FundChannel" => Request::FundChannel(FundchannelRequest::decode(p)?),
	"/cln.Node/GetRoute" => Request::GetRoute(GetrouteRequest::decode(p)?),
	"/cln.Node/ListForwards" => Request::ListForwards(ListforwardsRequest::decode(p)?),
	"/cln.Node/ListPays" => Request::ListPays(ListpaysRequest::decode(p)?),
	"/cln.Node/Ping" => Request::Ping(PingRequest::decode(p)?),
	"/cln.Node/SetChannel" => Request::SetChannel(SetchannelRequest::decode(p)?),
	"/cln.Node/SignMessage" => Request::SignMessage(SignmessageRequest::decode(p)?),
	"/cln.Node/FetchInvoice" => Request::FetchInvoice(FetchinvoiceRequest::decode(p)?),
	"/cln.Node/Stop" => Request::Stop(StopRequest::decode(p)?),
	"/cln.Node/ListClosedChannels" => Request::ListClosedChannels(ListclosedchannelsRequest::decode(p)?),
	"/cln.Node/StaticBackup" => Request::StaticBackup(StaticbackupRequest::decode(p)?),
	"/cln.Node/PreApproveInvoice" => Request::PreApproveInvoice(PreapproveinvoiceRequest::decode(p)?),
        uri => return Err(anyhow!("Unknown URI {}, can't decode payload", uri)),
    })
}
