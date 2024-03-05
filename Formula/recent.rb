class Recent < Formula
  desc "CLI for listing recently modified files"
  homepage "https://github.com/npty/homebrew-recent"
  url "https://github.com/npty/homebrew-recent/releases/download/0.1.0/recent-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "26628e0d1e2b63b39d343e37b90d6cd04b3a2c831c2dfb304be31914f5d94fc0"
  version "0.1.0"

  def install
    bin.install "recent"
  end
end
