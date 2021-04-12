// SPDX-License-Identifier: BSD-3-Clause
#ifndef LYREBIRD_PLAYBACK__HXX
#define LYREBIRD_PLAYBACK__HXX

#include <memory>
#include <libAudio.hxx>
#include "playlist/playlist.hxx"

namespace lyrebird
{
	using playlist::playlist_t;

	struct playbackThread_t final
	{
	private:
		playlist_t &playlist_;
		std::unique_ptr<audioFile_t> audioFile_{};
		const playlist::playlistEntry_t *song_{nullptr};

		void enterPlayState();
		void prepareNext();

	public:
		playbackThread_t(playlist_t &playlist) noexcept : playlist_{playlist} { }
		void operator ()() noexcept;

		void pausePlayback() noexcept;
		void stopPlayback() noexcept;

		auto *audioFile() const noexcept { return audioFile_.get(); }
		auto *song() const noexcept { return song_; }
	};
} // namespace lyrebird

#endif /*LYREBIRD_PLAYBACK__HXX*/
