# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: scheduler.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0fscheduler.proto\x12\tscheduler\"M\n\x10\x43hallengeRequest\x12(\n\x05scope\x18\x01 \x01(\x0e\x32\x19.scheduler.ChallengeScope\x12\x0f\n\x07node_id\x18\x02 \x01(\x0c\"&\n\x11\x43hallengeResponse\x12\x11\n\tchallenge\x18\x01 \x01(\x0c\"\x7f\n\x13RegistrationRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\x11\n\tbip32_key\x18\x02 \x01(\x0c\x12\r\n\x05\x65mail\x18\x03 \x01(\t\x12\x0f\n\x07network\x18\x04 \x01(\t\x12\x11\n\tchallenge\x18\x05 \x01(\x0c\x12\x11\n\tsignature\x18\x06 \x01(\x0c\"?\n\x14RegistrationResponse\x12\x13\n\x0b\x64\x65vice_cert\x18\x01 \x01(\t\x12\x12\n\ndevice_key\x18\x02 \x01(\t\"\"\n\x0fScheduleRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\"0\n\x0fNodeInfoRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\x0c\n\x04wait\x18\x02 \x01(\x08\"5\n\x10NodeInfoResponse\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\x10\n\x08grpc_uri\x18\x02 \x01(\t\"H\n\x0fRecoveryRequest\x12\x11\n\tchallenge\x18\x01 \x01(\x0c\x12\x11\n\tsignature\x18\x02 \x01(\x0c\x12\x0f\n\x07node_id\x18\x03 \x01(\x0c\";\n\x10RecoveryResponse\x12\x13\n\x0b\x64\x65vice_cert\x18\x01 \x01(\t\x12\x12\n\ndevice_key\x18\x02 \x01(\t*+\n\x0e\x43hallengeScope\x12\x0c\n\x08REGISTER\x10\x00\x12\x0b\n\x07RECOVER\x10\x01\x32\xfe\x02\n\tScheduler\x12M\n\x08Register\x12\x1e.scheduler.RegistrationRequest\x1a\x1f.scheduler.RegistrationResponse\"\x00\x12\x44\n\x07Recover\x12\x1a.scheduler.RecoveryRequest\x1a\x1b.scheduler.RecoveryResponse\"\x00\x12K\n\x0cGetChallenge\x12\x1b.scheduler.ChallengeRequest\x1a\x1c.scheduler.ChallengeResponse\"\x00\x12\x45\n\x08Schedule\x12\x1a.scheduler.ScheduleRequest\x1a\x1b.scheduler.NodeInfoResponse\"\x00\x12H\n\x0bGetNodeInfo\x12\x1a.scheduler.NodeInfoRequest\x1a\x1b.scheduler.NodeInfoResponse\"\x00\x62\x06proto3')

_CHALLENGESCOPE = DESCRIPTOR.enum_types_by_name['ChallengeScope']
ChallengeScope = enum_type_wrapper.EnumTypeWrapper(_CHALLENGESCOPE)
REGISTER = 0
RECOVER = 1


_CHALLENGEREQUEST = DESCRIPTOR.message_types_by_name['ChallengeRequest']
_CHALLENGERESPONSE = DESCRIPTOR.message_types_by_name['ChallengeResponse']
_REGISTRATIONREQUEST = DESCRIPTOR.message_types_by_name['RegistrationRequest']
_REGISTRATIONRESPONSE = DESCRIPTOR.message_types_by_name['RegistrationResponse']
_SCHEDULEREQUEST = DESCRIPTOR.message_types_by_name['ScheduleRequest']
_NODEINFOREQUEST = DESCRIPTOR.message_types_by_name['NodeInfoRequest']
_NODEINFORESPONSE = DESCRIPTOR.message_types_by_name['NodeInfoResponse']
_RECOVERYREQUEST = DESCRIPTOR.message_types_by_name['RecoveryRequest']
_RECOVERYRESPONSE = DESCRIPTOR.message_types_by_name['RecoveryResponse']
ChallengeRequest = _reflection.GeneratedProtocolMessageType('ChallengeRequest', (_message.Message,), {
  'DESCRIPTOR' : _CHALLENGEREQUEST,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.ChallengeRequest)
  })
_sym_db.RegisterMessage(ChallengeRequest)

ChallengeResponse = _reflection.GeneratedProtocolMessageType('ChallengeResponse', (_message.Message,), {
  'DESCRIPTOR' : _CHALLENGERESPONSE,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.ChallengeResponse)
  })
_sym_db.RegisterMessage(ChallengeResponse)

RegistrationRequest = _reflection.GeneratedProtocolMessageType('RegistrationRequest', (_message.Message,), {
  'DESCRIPTOR' : _REGISTRATIONREQUEST,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.RegistrationRequest)
  })
_sym_db.RegisterMessage(RegistrationRequest)

RegistrationResponse = _reflection.GeneratedProtocolMessageType('RegistrationResponse', (_message.Message,), {
  'DESCRIPTOR' : _REGISTRATIONRESPONSE,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.RegistrationResponse)
  })
_sym_db.RegisterMessage(RegistrationResponse)

ScheduleRequest = _reflection.GeneratedProtocolMessageType('ScheduleRequest', (_message.Message,), {
  'DESCRIPTOR' : _SCHEDULEREQUEST,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.ScheduleRequest)
  })
_sym_db.RegisterMessage(ScheduleRequest)

NodeInfoRequest = _reflection.GeneratedProtocolMessageType('NodeInfoRequest', (_message.Message,), {
  'DESCRIPTOR' : _NODEINFOREQUEST,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.NodeInfoRequest)
  })
_sym_db.RegisterMessage(NodeInfoRequest)

NodeInfoResponse = _reflection.GeneratedProtocolMessageType('NodeInfoResponse', (_message.Message,), {
  'DESCRIPTOR' : _NODEINFORESPONSE,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.NodeInfoResponse)
  })
_sym_db.RegisterMessage(NodeInfoResponse)

RecoveryRequest = _reflection.GeneratedProtocolMessageType('RecoveryRequest', (_message.Message,), {
  'DESCRIPTOR' : _RECOVERYREQUEST,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.RecoveryRequest)
  })
_sym_db.RegisterMessage(RecoveryRequest)

RecoveryResponse = _reflection.GeneratedProtocolMessageType('RecoveryResponse', (_message.Message,), {
  'DESCRIPTOR' : _RECOVERYRESPONSE,
  '__module__' : 'scheduler_pb2'
  # @@protoc_insertion_point(class_scope:scheduler.RecoveryResponse)
  })
_sym_db.RegisterMessage(RecoveryResponse)

_SCHEDULER = DESCRIPTOR.services_by_name['Scheduler']
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _CHALLENGESCOPE._serialized_start=619
  _CHALLENGESCOPE._serialized_end=662
  _CHALLENGEREQUEST._serialized_start=30
  _CHALLENGEREQUEST._serialized_end=107
  _CHALLENGERESPONSE._serialized_start=109
  _CHALLENGERESPONSE._serialized_end=147
  _REGISTRATIONREQUEST._serialized_start=149
  _REGISTRATIONREQUEST._serialized_end=276
  _REGISTRATIONRESPONSE._serialized_start=278
  _REGISTRATIONRESPONSE._serialized_end=341
  _SCHEDULEREQUEST._serialized_start=343
  _SCHEDULEREQUEST._serialized_end=377
  _NODEINFOREQUEST._serialized_start=379
  _NODEINFOREQUEST._serialized_end=427
  _NODEINFORESPONSE._serialized_start=429
  _NODEINFORESPONSE._serialized_end=482
  _RECOVERYREQUEST._serialized_start=484
  _RECOVERYREQUEST._serialized_end=556
  _RECOVERYRESPONSE._serialized_start=558
  _RECOVERYRESPONSE._serialized_end=617
  _SCHEDULER._serialized_start=665
  _SCHEDULER._serialized_end=1047
# @@protoc_insertion_point(module_scope)