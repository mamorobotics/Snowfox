#include "Connection.hpp"

void Connection::Send(int header, void* message)
{
    //std::string messageData = *message;
    std::string initialMsg = std::to_string(sizeof(message)) + "!" + std::to_string(header);
    initialMsg.insert(0, 32-initialMsg.size(), ' ');
    auto initSent = socket.send_to(asio::buffer(initialMsg, 32), remote_endpoint, 0);

    while(sizeof(message) > 65500){
        std::string temp = message.substr(0, 65500);
        message = message.substr(65500);
        auto msgSent = socket.send_to(asio::buffer(temp, 65500), remote_endpoint, 0);
    }
    if(sizeof(message) != 0){
        auto msgSent = socket.send_to(asio::buffer(message, sizeof(message)), remote_endpoint, 0);
    }
}

void Connection::Recieve() 
{
    int i = 0;
    int j=0;
    bool failedFrame = false;
    while (true)
    {
        j++;
        asio::error_code error;
        
        initial_buffer.resize(32);

        socket.receive_from(asio::buffer(initial_buffer), remote_endpoint, 0, error);

        std::string msg = std::string(initial_buffer.data());
        int index = msg.find("!");
        std::string size = msg.substr(0, index);
        std::string header = msg.substr(index + 1, msg.length() - (id.length() + 1));

        data_buffer.resize(0);

        int total_size = 0;
        while (total_size < size - 1)
        {
            std::vector<char> buf;
            buf.resize((size - total_size) > 65500 ? 65500 : (size - total_size));
            socket.receive_from(asio::buffer(buf), remote_endpoint, 0, error);
            total_size += (size - total_size) > 65500 ? 65500 : (size - total_size);
            data_buffer.insert(data_buffer.end(), buf.begin(), buf.end());
        }


        if (error.value()) SendError(error.message());

        data_buffer.resize(0);
    }
}

void Connection::HandleHandshake(){
    data_buffer.resize(32);
    asio::error_code error;
    socket.receive_from(asio::buffer(data_buffer), remote_endpoint, 0, error);
    if(data_buffer.data() != NULL){
        if(std::string(data_buffer.data()) != "0110"){
        } else { 
            connDetails.connectedIP = remote_endpoint.address().to_string();
            connDetails.connectedPort = "8080";
            connDetails.connectionStatus = "Connected";
            UI::Get()->setConnectionDetails(connDetails);
            Recieve();
        }
    }
}

Connection::~Connection()
{
}

Connection* Connection::Get()
{
	if (connection == nullptr)
		connection = new Connection();

	return connection;
}
