# frozen_string_literal: true

require_relative "helper"

class ClipboardTest < Minitest::Test
  include NativeTestHelpers

  def test_app_clipboard_and_text_fields_share_a_private_headless_clipboard
    skip_without_real_binary
    Dir.mktmpdir("scarpe-clipboard") do |dir|
      board = File.join(dir, "clipboard.txt")
      File.write(board, "outside the renderer")
      run = run_real(<<~APP, env: { "SPEC_CLIPBOARD_FILE" => board }, test_code: <<~TEST)
        Shoes.app { edit_box }
      APP
        app = Shoes.APPS.first
        assert_equal "", app.clipboard, "headless apps start with a private clipboard"
        text = "آدابُ العلم\nصفحة ٧٢"
        assert_equal text, app.public_send(:clipboard=, text)
        assert_equal text, app.clipboard

        click_on edit_box
        press_key :control_v
        assert_equal text, edit_box.text
        press_key :control_a
        type_text "من الحقل"
        press_key :control_a
        press_key :control_c
        assert_equal "من الحقل", app.clipboard

        app.clipboard = ""
        assert_equal "", app.clipboard
      TEST
      assert_spec_passed(run)
      assert_equal "outside the renderer", File.read(board), "native clipboard calls never run pbcopy, pbpaste or xclip"
    end
  end
end
