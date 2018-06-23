#ifndef ARCHE_SERVER_H_
#define ARCHE_SERVER_H_

#include "common.h"

using Poco::DateTimeFormat;
using Poco::DateTimeFormatter;
using Poco::ThreadPool;
using Poco::Timestamp;
using Poco::Net::HTTPRequestHandler;
using Poco::Net::HTTPRequestHandlerFactory;
using Poco::Net::HTTPServer;
using Poco::Net::HTTPServerParams;
using Poco::Net::HTTPServerRequest;
using Poco::Net::HTTPServerResponse;
using Poco::Net::ServerSocket;
using Poco::Util::Application;
using Poco::Util::HelpFormatter;
using Poco::Util::Option;
using Poco::Util::OptionCallback;
using Poco::Util::OptionSet;
using Poco::Util::ServerApplication;

namespace arche {
namespace server {

class Handler : public HTTPRequestHandler {
public:
  Handler(const std::string &format);
  void handleRequest(HTTPServerRequest &request, HTTPServerResponse &response);

private:
  std::string _format;
};

class HandlerFactory : public HTTPRequestHandlerFactory {
public:
  HandlerFactory(const std::string &format);
  HTTPRequestHandler *createRequestHandler(const HTTPServerRequest &request);

private:
  std::string _format;
};

class Server : public ServerApplication {
public:
  Server();
  ~Server();

protected:
  void initialize(Application &self);
  void uninitialize();
  void defineOptions(OptionSet &options);
  void handleHelp(const std::string &name, const std::string &value);
  void handleVersion(const std::string &name, const std::string &value);
  int main(const std::vector<std::string> &args);

private:
  bool _running;
};

} // namespace server
} // namespace arche

#endif
