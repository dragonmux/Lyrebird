// SPDX-License-Identifier: BSD-3-Clause
#ifndef PLAYLIST_PLAYLIST__HXX
#define PLAYLIST_PLAYLIST__HXX

#include <cstdint>
#include <atomic>
#include <vector>
#include "playlist/entry.hxx"

namespace lyrebird::playlist
{
	enum struct playState_t : uint8_t
	{
		stopped,
		playing,
		paused
	};

	struct playlist_t final
	{
	private:
		playState_t state_{playState_t::stopped};
		std::atomic<std::size_t> index_{0};
		std::vector<playlistEntry_t> entries_{};

	public:
		playlist_t() noexcept = default;

		auto state() const noexcept { return state_; }
		void state(playState_t state) noexcept { state_ = state; }
		auto playing() const noexcept { return state_ == playState_t::playing; }

		auto &currentEntry() const noexcept { return entries_[index_]; }
		const playlistEntry_t &operator [](std::size_t index) const;
		void operator ++() noexcept;
		void operator --() noexcept;
	};
} // namespace lyrebird::playlist

#endif /*PLAYLIST_PLAYLIST__HXX*/
