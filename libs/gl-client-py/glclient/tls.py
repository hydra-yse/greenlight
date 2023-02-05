from . import glclient as native
from typing import Optional, List, Union, Tuple, Iterable, SupportsIndex, Type, Any, TypeVar

class TlsConfig(object):
    def __init__(self) -> None:
        # We wrap the TlsConfig since some calls cannot yet be routed
        # through the rust library (streaming calls)
        self.inner = native.TlsConfig()
        self.ca: Optional[bytes] = None
        self.id: Tuple[Optional[bytes], Optional[bytes]] = (None, None)

    def identity(self, cert_pem: Union[str, bytes], key_pem: Union[str, bytes]) -> "TlsConfig":
        if isinstance(cert_pem, str):
            cert_pem = cert_pem.encode('ASCII')

        if isinstance(key_pem, str):
            key_pem = key_pem.encode('ASCII')

        c = TlsConfig()
        c.inner = self.inner.identity(cert_pem, key_pem)
        c.ca = self.ca
        c.id = (cert_pem, key_pem)
        return c

    def with_ca_certificate(self, ca: Union[str, bytes]) -> "TlsConfig":
        if isinstance(ca, str):
            ca = ca.encode('ASCII')

        c = TlsConfig()
        c.inner = self.inner.with_ca_certificate(ca)
        c.ca = ca
        c.id = self.id
        return c