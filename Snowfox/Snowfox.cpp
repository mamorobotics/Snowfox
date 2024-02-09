#pragma once

#include <iostream>
#include <chrono>

#include "Connection.hpp"

using namespace std::chrono;

Connection* Connection::connection = new Connection();

int main()
{
	Connection* conn = Connection::Get();

	std::string message = "Bonjour!";
	conn->Send(11, &message);

	bool firstFrame = true;

	std::thread networkThread(&Connection::HandleHandshake, conn);
	networkThread.detach();

	while (true) {
		auto start = high_resolution_clock::now();
			
		if (firstFrame) {
			firstFrame = false;
		}
		auto stop = high_resolution_clock::now();
		int duration = duration_cast<microseconds>(stop - start).count();
	}
	
	return 0;
}
