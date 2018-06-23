#include "server.h"

using arche::server::Handler;
using arche::server::HandlerFactory;
using arche::server::Server;

Handler::Handler(const std::string &format) : _format(format) {}

void Handler::handleRequest(HTTPServerRequest &request,
                            HTTPServerResponse &response) {
  Application &app = Application::instance();
  app.logger().information("Request from " +
                           request.clientAddress().toString());

  Timestamp now;
  std::string dt(DateTimeFormatter::format(now, _format));

  response.setChunkedTransferEncoding(true);
  response.setContentType("text/html");

  std::ostream &ostr = response.send();
  ostr << "<html><head><title>Server powered by "
          "POCO C++ Libraries</title>";
  ostr << "<meta http-equiv=\"refresh\" content=\"1\"></head>";
  ostr << "<body><p style=\"text-align: center; "
          "font-size: 48px;\">";
  ostr << dt;
  ostr << "</p></body></html>";
}

HandlerFactory::HandlerFactory(const std::string &format) : _format(format) {}

HTTPRequestHandler *
HandlerFactory::createRequestHandler(const HTTPServerRequest &request) {
  if (request.getURI() == "/")
    return new Handler(_format);
  else
    return NULL;
}

Server::Server() : _running(true) {}

Server::~Server() {}

void Server::initialize(Application &self) {
  loadConfiguration();
  ServerApplication::initialize(self);
}

void Server::uninitialize() { ServerApplication::uninitialize(); }

void Server::defineOptions(OptionSet &options) {
  ServerApplication::defineOptions(options);

  options.addOption(
      Option("help", "h", "Display argument help information")
          .required(false)
          .repeatable(false)
          .callback(OptionCallback<Server>(this, &Server::handleHelp)));
  options.addOption(
      Option("version", "v", "Print ARCHE version")
          .required(false)
          .repeatable(false)
          .callback(OptionCallback<Server>(this, &Server::handleVersion)));
}

void Server::handleVersion(const std::string &name, const std::string &value) {
  std::cout << commandName() << ' ' << ARCHE_VERSION_MAJOR << '.'
            << ARCHE_VERSION_MINOR << '.' << ARCHE_VERSION_PATCH << " ("
            << ARCHE_VERSION_GIT << ')' << " by " << ARCHE_AUTHOR_NAME << '<'
            << ARCHE_AUTHOR_EMAIL << '>' << " at " << ARCHE_BUILD_TIME
            << " with " << ARCHE_COPYRIGHT << std::endl;
  stopOptionsProcessing();
  _running = false;
}

void Server::handleHelp(const std::string &name, const std::string &value) {
  HelpFormatter helpFormatter(options());
  helpFormatter.setCommand(commandName());
  helpFormatter.setUsage("OPTIONS");
  helpFormatter.setHeader(ARCHE_DESCRIPTION);
  helpFormatter.setFooter(ARCHE_HOMEAGE);
  helpFormatter.format(std::cout);
  stopOptionsProcessing();
  _running = false;
}

int Server::main(const std::vector<std::string> &args) {
  if (_running) {
    unsigned short port = (unsigned short)config().getInt("Server.port", 8080);
    std::string format(
        config().getString("Server.format", DateTimeFormat::SORTABLE_FORMAT));

    ServerSocket svs(port);
    HTTPServer srv(new HandlerFactory(format), svs, new HTTPServerParams);
    srv.start();
    waitForTerminationRequest();
    srv.stop();
  }
  return Application::EXIT_OK;
}
