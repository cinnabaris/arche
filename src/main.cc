#include "app.h"

int main(int argc, char *argv[]) {
  arche::server::Server app;
  return app.run(argc, argv);
}
