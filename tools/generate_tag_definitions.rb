require 'net/http'
require 'nokogiri'

OUTPUT_FILE = File.join(File.dirname(File.expand_path(__FILE__)), '..', 'src', 'arw_file', 'ifd', 'tag.rs')
OUTPUT_DELIMITER = '// Auto-generated code below'
OUTPUT_REGEX = /^#{Regexp.escape(OUTPUT_DELIMITER)}.*/m
SOURCE_URIS = %w[
  http://www.awaresystems.be/imaging/tiff/tifftags/baseline.html
  http://www.awaresystems.be/imaging/tiff/tifftags/extension.html
  http://www.awaresystems.be/imaging/tiff/tifftags/private.html
  http://www.awaresystems.be/imaging/tiff/tifftags/privateifd/exif.html
  http://www.awaresystems.be/imaging/tiff/tifftags/privateifd/gps.html
].map { |url| URI.parse(url) }

tags = {}
SOURCE_URIS.each do |uri|
  Nokogiri::HTML(Net::HTTP.get(uri)).xpath('/html/body/table/tbody/tr[4]/td[7]/table/tbody/tr').each do |tr_node|
    id, _hex, label, description = tr_node.children.map(&:text)
    id = Integer(id)
    raise "Tag already present #{id} #{label} #{description} - #{tags[id].inspect}" if tags[id]
    tags[id] = { label: label, description: description }
  end
end

rust_static_src = [OUTPUT_DELIMITER]
rust_static_src << "// #{Time.now.to_s} "
rust_static_src << ""
rust_static_src << "lazy_static! { "
rust_static_src << "    pub static ref TAGS : HashMap<u16, Tag> = {"
rust_static_src << "        let mut m = HashMap::new();"

tags.each do |id, tag|
  ifd = !!(tag[:label] =~ /IFD/)
  rust_static_src << "        m.insert(#{id}, Tag {id: #{id}, ifd: #{ifd.inspect}, label: String::from(\"#{tag[:label]}\"), description: String::from(\"#{tag[:description]}\")});"
end

rust_static_src << "        m"
rust_static_src << "    };" # value
rust_static_src << "}" # lazy_static!

rust_code = rust_static_src.join("\n")

current_source = File.read(OUTPUT_FILE)
current_source.sub!(OUTPUT_REGEX, rust_code)
File.open(OUTPUT_FILE, "w") { |f| f.write(current_source) }