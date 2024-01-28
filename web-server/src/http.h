#include <stdio.h>
#include <string.h>

int get_file_size(const char *filename) {
    FILE *file = fopen(filename, "rb"); // Open the file in binary mode
    if (file == NULL) {
        perror("Error opening file");
        return -1; // Error opening the file
    }

    fseek(file, 0, SEEK_END); // Move the file pointer to the end of the file
    long size = ftell(file);  // Get the current position, which is the file size

    fclose(file); // Close the file

    return size;
}

char* serve_http_request(char* requestLine, char* headers, char* requestBody){
  printf("Serving HTTP request....\n");

  char req_type[10], path[1024], ans[1024];
  int i = 0;
  while( requestLine[i] !=  ' ' ){
    req_type[i] = requestLine[i];
    //memcpy( (char*)req_type + i, (char*)requestLine + i, sizeof(char));
    i += 1;
  }
  req_type[i] = '\0';
  //char* rq = (char *)(path + i);
  //*rq = '\0';

  i += 1;
  int j = 0;
  while( requestLine[i] != ' '){
    path[j] = requestLine[i];
    i += 1;
    j += 1;
  }
  path[j] = '\0';
  //char* pt = (char *)(path + j);
  //*pt = '\0';

  char* sp = ans;

  printf("req_type: %s, path: %s\n", (char *)req_type, (char *)path);

  if ( strcmp(req_type, "GET") == 0 ){
    if ( strcmp(path, "/") == 0 ){
      FILE* f = fopen("pages/index.html", "r");
      char message[1024], buffer[1024];

      sprintf(message, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: %d\r\n\r\n", get_file_size("pages/index.html"));
      strcat(ans, message);

      while (fgets(buffer, sizeof(buffer), f) != NULL){
        strcat(ans, buffer);
      }

      fclose(f);
    } else {
      fprintf(stderr, "error opening file to read HTML!");
    }
  } 

  return sp;
}
