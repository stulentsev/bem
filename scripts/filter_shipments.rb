#!/usr/bin/env ruby

require "json"
require "open3"

MAX_BATCH = 100

def usage!
  warn "Usage: filter_shipments.rb <references_file>"
  exit 1
end

references_file = ARGV.shift || usage!
usage! unless ARGV.empty?

unless File.file?(references_file)
  warn "File not found: #{references_file}"
  exit 1
end

references = File.readlines(references_file, chomp: true)
  .map { |line| line.strip }
  .reject(&:empty?)

if references.empty?
  warn "No references found in #{references_file}"
  exit 1
end

def run_getref(batch)
  cmd = ["bem", "getref", *batch]
  stdout, stderr, status = Open3.capture3(*cmd)
  unless status.success?
    warn "Command failed: #{cmd.join(" ")}"
    warn stderr
    exit status.exitstatus || 1
  end
  stdout
end

def stops_references(element)
  stops = element.dig("transformedContent", "Stops")
  return [] unless stops.is_a?(Array) && !stops.empty?

  first_stop = stops.first
  references = first_stop["References"]
  return [] unless references.is_a?(Array)

  references
end

references.each_slice(MAX_BATCH) do |batch|
  raw = run_getref(batch)
  begin
    data = JSON.parse(raw)
  rescue JSON::ParserError => e
    warn "Failed to parse JSON from bem getref: #{e.message}"
    exit 1
  end

  unless data.is_a?(Array)
    warn "Unexpected response (expected array): #{raw}"
    exit 1
  end

  data.each do |element|
    ref_id = element["referenceID"]
    next unless ref_id

    ship_ref_count = stops_references(element)
      .count { |ref| ref["ReferenceName"] == "Shipments" }

    next if ship_ref_count < 2

    puts ref_id
  end
end

