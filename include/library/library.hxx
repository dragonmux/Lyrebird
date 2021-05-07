// SPDX-License-Identifier: BSD-3-Clause
#ifndef LIBRARY_LIBRARY__HXX
#define LIBRARY_LIBRARY__HXX

namespace lyrebird::library
{
	struct library_t final
	{
	private:

	public:
		void loadCache() noexcept;
	};

	extern library_t mediaLibrary;
} // namespace lyrebird::library

#endif /*LIBRARY_LIBRARY__HXX*/
