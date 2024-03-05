class Recent < Formula
  desc "CLI for listing recently modified files"
  homepage "https://github.com/npty/homebrew-recent"
  url "https://github.com/npty/homebrew-recent/releases/download/v0.1.0/recent-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "a48c8347275c81a97a378bea61b38bf6a8bd3cad2fb7aa2a61ff483928374b15"
  version "0.1.0"

  def install
    bin.install "recent"
  end
end
