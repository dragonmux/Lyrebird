// SPDX-License-Identifier: BSD-3-Clause
#include "../include/playback.hxx"

namespace lyrebird
{
	using namespace playlist;

	void playbackThread_t::operator ()() noexcept try
	{
		enterPlayState();
		while (true)
		{
			audioFile_->play();
			prepareNext();
		}
	}
	catch (...)
	{
	}

	void playbackThread_t::enterPlayState()
	{
		const auto state{playlist_.state()};
		playlist_.state(playState_t::playing);
		if (state == playState_t::stopped)
			prepareNext();
	}

	void playbackThread_t::prepareNext()
	{
		++playlist_;
		song_ = &playlist_.currentEntry();
		std::unique_ptr<audioFile_t> file{audioFile_t::openR(song_->fileName().data())};
		if (!file)
			// For now, throw this..
			// However, we actually want to throw some kind of libAudio failure exception instead
			throw std::bad_alloc{};
		audioFile_ = std::move(file);
	}

	void playbackThread_t::pausePlayback() noexcept
	{
		playlist_.state(playState_t::paused);
		audioFile_->pause();
	}

	void playbackThread_t::stopPlayback() noexcept
	{
		playlist_.state(playState_t::stopped);
		audioFile_->stop();
	}
} // namespace lyrebird
