-- Part 1
SELECT
  SUM(
    CAST(
      SUBSTRING(calibration_value FROM '[0-9]') ||
      COALESCE(REVERSE(SUBSTRING(REVERSE(calibration_value) FROM '[0-9]')), '')
      AS INTEGER
    )
  ) AS decoded_calibration_value
FROM calibration;

-- Part 2
SELECT
  SUM(
    CAST(
      SUBSTRING(replaced_calibration_value FROM '[0-9]') ||
      COALESCE(REVERSE(SUBSTRING(REVERSE(replaced_calibration_value) FROM '[0-9]')), '')
      AS INTEGER
    )
  ) AS decoded_calibration_value
FROM (
  SELECT
    REPLACE(
        REPLACE(
            REPLACE(
                REPLACE(
                    REPLACE(
                        REPLACE(
                            REPLACE(
                                REPLACE(
                                    REPLACE(calibration_value, 'one', 'one1one'),
                                'two', 'two2two'),
                            'three', 'three3three'),
                        'four', 'four4four'),
                    'five', 'five5five'),
                'six', 'six6six'),
            'seven', 'seven7seven'),
        'eight', 'eight8eight'),
    'nine', 'nine9nine')
    AS replaced_calibration_value
  FROM calibration
);
