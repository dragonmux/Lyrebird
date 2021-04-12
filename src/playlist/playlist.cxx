// SPDX-License-Identifier: BSD-3-Clause
#include <stdexcept>
#include "playlist/playlist.hxx"

namespace lyrebird::playlist
{
	void playlist_t::operator ++() noexcept
	{
		++index_;
		if (index_ >= entries_.size())
			index_ = 0;
	}

	void playlist_t::operator --() noexcept
	{
		if (index_ == 0)
			index_ = entries_.size();
		--index_;
	}

	const playlistEntry_t &playlist_t::operator [](const std::size_t index) const
	{
		if (index >= entries_.size())
			throw std::out_of_range{"playlist index out of range"};
		return entries_[index];
	}
} // namespace lyrebird::playlist
