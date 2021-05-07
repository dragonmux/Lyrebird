// SPDX-License-Identifier: BSD-3-Clause
#include <thread>
#include "library/library.hxx"
#include "config.hxx"

using lyrebird::library::mediaLibrary;

int main(int, char **)
{
	// Initiate background loading of the media library cache
	std::thread{[]() noexcept { mediaLibrary.loadCache(); }}.detach();
	return 0;
}
