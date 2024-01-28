#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <netdb.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <curl/curl.h>
#include <arpa/inet.h>
#include <curl/curl.h>

#define PORT "3490" // the port client will be connecting to 

#define MAXDATASIZE 100 // max number of bytes we can get at once 


// get sockaddr, IPv4 or IPv6:
void *get_in_addr(struct sockaddr *sa)
{
	if (sa->sa_family == AF_INET) {
		return &(((struct sockaddr_in*)sa)->sin_addr);
	}

	return &(((struct sockaddr_in6*)sa)->sin6_addr);
}

int main(int argc, char *argv[]){
	int sockfd;  
	char buf[MAXDATASIZE];
	struct addrinfo hints, *servinfo, *p;
	int rv;
	char s[INET6_ADDRSTRLEN];
  
	memset(&hints, 0, sizeof hints);
	hints.ai_family = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;

	if ((rv = getaddrinfo("localhost", PORT, &hints, &servinfo)) != 0) {
		fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(rv));
		return 1;
	}

	// loop through all the results and connect to the first we can
	for(p = servinfo; p != NULL; p = p->ai_next) {
		if ((sockfd = socket(p->ai_family, p->ai_socktype,
				p->ai_protocol)) == -1) {
			perror("client: socket");
			continue;
		} else {
      printf("Generated socket with fd number: %d\n", sockfd);
    }

		if (connect(sockfd, p->ai_addr, p->ai_addrlen) == -1) {
			perror("client: connect");
			close(sockfd);
			continue;
		} else {
      printf("Connected sockfd to remote address and port.\n");
    }

		break;
	}

	if (p == NULL) {
		fprintf(stderr, "client: failed to connect\n");
		return 2;
	}

	inet_ntop(p->ai_family, get_in_addr((struct sockaddr *)p->ai_addr),
			s, sizeof s);
	printf("client: connecting to %s\n", s);

	freeaddrinfo(servinfo); // all done with this structure

  ssize_t numbytes;
  
  int result = system("curl http://localhost:3490");
  if (result == -1){
    fprintf(stderr, "Did not execute GET request properly.");
  } else {
    fprintf(stdout, "GET request completed properly.");
  }
  /*
  char* msg = "GET:/api/data\r\nContent-Type: application/json\r\n\r\n{'key1', 'value1'}\r\n\r\n\0";
	if ((numbytes = send(sockfd, (void *)msg, strlen(msg), 0)) == -1) {
	    perror("send");
	    exit(1);
	}

  printf("Number of bytes sent: %d\n", (int)numbytes);
  printf("Sent message: %s\n", msg);
  */
	close(sockfd);

	return 0;
}
