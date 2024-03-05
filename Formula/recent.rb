class Recent < Formula
  desc "CLI for listing recently modified files"
  homepage "https://github.com/npty/homebrew-recent"
  url "https://github.com/npty/homebrew-recent/releases/download/0.1.0/recent-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "4c6b05187a2701047922cec2f91e96069bbe935e"
  version "0.1.0"

  def install
    bin.install "recent"
  end
end
