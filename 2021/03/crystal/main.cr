diagnostic_report = File.read("input.txt").split('\n')

p "We have a power consumption of #{power_consumption(diagnostic_report)}"
p "We have a life support rating of #{oxygen_rating(diagnostic_report).to_i(2) * co2_rating(diagnostic_report).to_i(2)}"

def co2_rating(report)
  co2_ratings = report.dup
  (1..report.first.size).each_with_index do |_, index|
    least_common_bit = co2_ratings.count { |rating| rating[index] == '0' } <= co2_ratings.size / 2 ? '0' : '1'
    co2_ratings.select! { |rating| rating[index] == least_common_bit } if co2_ratings.size > 1
  end
  co2_ratings.first
end

def oxygen_rating(report)
  oxygen_ratings = report.dup
  (1..report.first.size).each_with_index do |_, index|
    most_common_bit = oxygen_ratings.count { |rating| rating[index] == '1' } >= oxygen_ratings.size / 2 ? '1' : '0'
    oxygen_ratings.select! { |rating| rating[index] == most_common_bit } if oxygen_ratings.size > 1
  end
  oxygen_ratings.first
end

def power_consumption(report)
  consumption = { "gamma_rate" => "", "epsilon_rate" => ""}
  (1..report.first.size).each_with_index do |_, index|
    count_one = report.count { |diagnostic| diagnostic[index] == '1' }
    consumption["gamma_rate"] += count_one > (report.size / 2) ? "1" : "0"
    consumption["epsilon_rate"] += count_one > (report.size / 2) ? "0" : "1"
  end
  consumption["gamma_rate"].to_i(2) * consumption["epsilon_rate"].to_i(2)
end
