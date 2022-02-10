initSidebarItems({"constant":[["FDFLAGS_APPEND","Append mode: Data written to the file is always appended to the file’s end."],["FDFLAGS_DSYNC","Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized."],["FDFLAGS_NONBLOCK","Non-blocking mode."],["FDFLAGS_RSYNC","Synchronized read I/O operations."],["FDFLAGS_SYNC","Write according to synchronized I/O file integrity completion. In addition to synchronizing the data stored in the file, the implementation may also synchronously update the file’s metadata."]],"enum":[["AddressFamily",""],["AiFlags",""],["AiProtocol",""],["IpAddr","An IP address, either IPv4 or IPv6."],["Shutdown","Possible values which can be passed to the [`TcpStream::shutdown`] method."],["SocketAddr","An internet socket address, either IPv4 or IPv6."],["SocketType",""]],"fn":[["set_fdflag","Set the flags associated with a file descriptor."],["sock_accept",""],["sock_bind",""],["sock_close",""],["sock_connect",""],["sock_getaddrinfo",""],["sock_getlocaladdr",""],["sock_getpeeraddr",""],["sock_listen",""],["sock_open",""],["sock_recv",""],["sock_recv_from",""],["sock_send",""],["sock_send_to",""],["sock_shutdown",""],["unset_fdflag","Unset the flags associated with a file descriptor."]],"mod":[["poll",""]],"struct":[["Incoming",""],["IovecRead",""],["IovecWrite",""],["Ipv4Addr","An IPv4 address."],["TcpListener",""],["TcpStream",""],["UdpSocket",""],["WasiAddress",""],["WasiAddrinfo",""],["WasiSockaddr",""]],"trait":[["AsRawFd",""],["ToSocketAddrs","A trait for objects which can be converted or resolved to one or more [`SocketAddr`] values."]],"type":[["Fdflags","File descriptor flags"]]});