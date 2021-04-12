// SPDX-License-Identifier: BSD-3-Clause
#ifndef PLAYLIST_ENTRY__HXX
#define PLAYLIST_ENTRY__HXX

#include <string_view>

namespace lyrebird::playlist
{
	struct playlistEntry_t final
	{
	private:
		std::string_view fileName_;

	public:
		auto fileName() const noexcept { return fileName_; }
	};
} // namespace lyrebird::playlist

#endif /*PLAYLIST_ENTRY__HXX*/
