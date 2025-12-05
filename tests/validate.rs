use std::process::Command;

use assert_cmd::cargo::cargo_bin_cmd;
use bc_ur::UREncodable;
use chrono::TimeZone;
use dcbor::Date;
use indoc::indoc;
use provenance_mark::{
    ProvenanceMark, ProvenanceMarkGenerator, ProvenanceMarkResolution,
};
use tempfile::TempDir;

/// A macro to assert that two values are equal, printing them if they are not,
/// including newlines and indentation they may contain. This macro is useful
/// for debugging tests where you want to see the actual and expected values
/// when they do not match.
#[macro_export]
macro_rules! assert_actual_expected {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val);
                }
            }
        }
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val, $($arg)+);
                }
            }
        }
    };
}

fn create_test_marks(
    count: usize,
    resolution: ProvenanceMarkResolution,
    passphrase: &str,
) -> Vec<ProvenanceMark> {
    provenance_mark::register_tags();

    let mut generator =
        ProvenanceMarkGenerator::new_with_passphrase(resolution, passphrase);
    let calendar = chrono::Utc;

    (0..count)
        .map(|i| {
            let date = Date::from_datetime(
                calendar
                    .with_ymd_and_hms(2023, 6, 20, 12, 0, 0)
                    .single()
                    .unwrap()
                    .checked_add_signed(chrono::Duration::days(i as i64))
                    .unwrap(),
            );
            generator.next(date, None::<String>)
        })
        .collect()
}

fn marks_to_ur_strings(marks: &[ProvenanceMark]) -> Vec<String> {
    marks.iter().map(|m| m.ur().to_string()).collect()
}

fn run_validate_command(ur_strings: &[String], warn: bool) -> (bool, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_provenance"));
    cmd.arg("validate");

    if warn {
        cmd.arg("--warn");
    }

    for ur in ur_strings {
        cmd.arg(ur);
    }

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}{}", stdout, stderr);

    (output.status.success(), combined)
}

mod validate_command {
    use super::*;

    #[test]
    fn test_validate_single_valid_mark() {
        let marks = create_test_marks(1, ProvenanceMarkResolution::Low, "test");
        let ur_strings = marks_to_ur_strings(&marks);

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should succeed with no output (not interesting)
        assert!(success, "Command should succeed");
        assert!(
            output.trim().is_empty() || !output.contains("error"),
            "Output: {}",
            output
        );
    }

    #[test]
    fn test_validate_valid_sequence() {
        let marks =
            create_test_marks(10, ProvenanceMarkResolution::Low, "test");
        let ur_strings = marks_to_ur_strings(&marks);

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should succeed with no output (not interesting)
        assert!(success, "Command should succeed");
        assert!(
            output.trim().is_empty() || !output.contains("error"),
            "Output: {}",
            output
        );
    }

    #[test]
    fn test_validate_with_duplicates() {
        let marks = create_test_marks(3, ProvenanceMarkResolution::Low, "test");
        let mut ur_strings = marks_to_ur_strings(&marks);

        // Add duplicates
        ur_strings.push(ur_strings[0].clone());
        ur_strings.push(ur_strings[1].clone());

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should succeed - duplicates are removed, leaving a perfect chain
        assert!(success, "Command should succeed after deduplication");
        assert!(
            output.trim().is_empty() || !output.contains("error"),
            "Output: {}",
            output
        );
    }
    #[test]
    fn test_validate_with_gap() {
        let marks = create_test_marks(5, ProvenanceMarkResolution::Low, "test");

        // Create a gap by removing mark at index 2
        let marks_with_gap = vec![
            marks[0].clone(),
            marks[1].clone(),
            marks[3].clone(), // Gap: skips seq 2
            marks[4].clone(),
        ];
        let ur_strings = marks_to_ur_strings(&marks_with_gap);

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should fail with gap report
        assert!(!success, "Command should fail with gap");

        #[rustfmt::skip]
        assert_actual_expected!(output.trim(), indoc! {r#"
            Error: Validation failed with issues:
            Total marks: 4
            Chains: 1

            Chain 1: b16a7cbd
              0: f057c8c4 (genesis mark)
              1: 1b806d6c
              3: 761a5e74 (gap: 2 missing)
              4: 42d12de5
        "#}.trim());
    }
    #[test]
    fn test_validate_with_gap_warn_flag() {
        let marks = create_test_marks(5, ProvenanceMarkResolution::Low, "test");

        // Create a gap
        let marks_with_gap = vec![
            marks[0].clone(),
            marks[1].clone(),
            marks[3].clone(),
            marks[4].clone(),
        ];
        let ur_strings = marks_to_ur_strings(&marks_with_gap);

        let (success, output) = run_validate_command(&ur_strings, true);

        // Should succeed with --warn flag
        assert!(success, "Command should succeed with --warn flag");

        #[rustfmt::skip]
        assert_actual_expected!(output.trim(), indoc! {r#"
            Total marks: 4
            Chains: 1

            Chain 1: b16a7cbd
              0: f057c8c4 (genesis mark)
              1: 1b806d6c
              3: 761a5e74 (gap: 2 missing)
              4: 42d12de5
        "#}.trim());
    }
    #[test]
    fn test_validate_multiple_chains() {
        let marks1 =
            create_test_marks(3, ProvenanceMarkResolution::Low, "alice");
        let marks2 = create_test_marks(3, ProvenanceMarkResolution::Low, "bob");

        let mut all_marks = marks1;
        all_marks.extend(marks2);
        let ur_strings = marks_to_ur_strings(&all_marks);

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should fail (multiple chains is an issue)
        assert!(!success, "Command should fail with multiple chains");

        #[rustfmt::skip]
        assert_actual_expected!(output.trim(), indoc! {r#"
            Error: Validation failed with issues:
            Total marks: 6
            Chains: 2

            Chain 1: 7a9c3f5e
              0: 0d6e0afd (genesis mark)
              1: 6cd504e7
              2: dc07895c

            Chain 2: a33e10de
              0: c2a985ff (genesis mark)
              1: 5567cd24
              2: f759ad4c
        "#}.trim());
    }

    #[test]
    fn test_validate_missing_genesis() {
        let marks = create_test_marks(5, ProvenanceMarkResolution::Low, "test");

        // Remove genesis mark (index 0)
        let marks_no_genesis: Vec<_> = marks.into_iter().skip(1).collect();
        let ur_strings = marks_to_ur_strings(&marks_no_genesis);

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should fail (missing genesis)
        assert!(!success, "Command should fail with missing genesis");

        #[rustfmt::skip]
        assert_actual_expected!(output.trim(), indoc! {r#"
            Error: Validation failed with issues:
            Total marks: 4
            Chains: 1

            Chain 1: b16a7cbd
              Warning: No genesis mark found
              1: 1b806d6c
              2: b292f357
              3: 761a5e74
              4: 42d12de5
        "#}.trim());
    }
    #[test]
    fn test_validate_invalid_ur() {
        let ur_strings = vec!["ur:invalid/abcd".to_string()];

        let (success, output) = run_validate_command(&ur_strings, false);

        assert!(!success, "Command should fail with invalid UR");
        assert!(
            output.contains("Failed to parse UR") || output.contains("error"),
            "Output should mention parse error: {}",
            output
        );
    }

    #[test]
    fn test_validate_multiple_sequences_in_chain() {
        let marks = create_test_marks(7, ProvenanceMarkResolution::Low, "test");

        // Create multiple gaps
        let marks_with_gaps = vec![
            marks[0].clone(), // Sequence 1: [0,1]
            marks[1].clone(),
            marks[3].clone(), // Sequence 2: [3,4]
            marks[4].clone(),
            marks[6].clone(), // Sequence 3: [6]
        ];
        let ur_strings = marks_to_ur_strings(&marks_with_gaps);

        let (success, output) = run_validate_command(&ur_strings, false);

        // Should fail (multiple sequences)
        assert!(!success, "Command should fail with multiple sequences");

        #[rustfmt::skip]
        assert_actual_expected!(output.trim(), indoc! {r#"
            Error: Validation failed with issues:
            Total marks: 5
            Chains: 1

            Chain 1: b16a7cbd
              0: f057c8c4 (genesis mark)
              1: 1b806d6c
              3: 761a5e74 (gap: 2 missing)
              4: 42d12de5
              6: 8a9b06e1 (gap: 5 missing)
        "#}.trim());
    }
    #[test]
    fn test_validate_format_output() {
        let marks = create_test_marks(5, ProvenanceMarkResolution::Low, "test");

        // Create a gap
        let marks_with_gap =
            vec![marks[0].clone(), marks[1].clone(), marks[3].clone()];
        let ur_strings = marks_to_ur_strings(&marks_with_gap);

        let (success, output) = run_validate_command(&ur_strings, true);

        assert!(success, "Command should succeed with --warn flag");

        #[rustfmt::skip]
        assert_actual_expected!(output.trim(), indoc! {r#"
            Total marks: 3
            Chains: 1

            Chain 1: b16a7cbd
              0: f057c8c4 (genesis mark)
              1: 1b806d6c
              3: 761a5e74 (gap: 2 missing)
        "#}.trim());
    }
}

mod quartile_directory_workflow {
    use super::*;

    #[test]
    fn test_new_next_validate_dir() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let chain_path = temp_dir.path().join("test-chain");

        // Step 1: Create a new chain with Quartile resolution using a fixed
        // date
        let new_output = cargo_bin_cmd!("provenance")
            .arg("new")
            .arg(&chain_path)
            .arg("--resolution")
            .arg("quartile")
            .arg("--date")
            .arg("2023-06-20T12:00:00Z")
            .arg("--comment")
            .arg("Test genesis mark")
            .arg("--quiet")
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let new_output_str = String::from_utf8_lossy(&new_output);
        assert!(
            !new_output_str.is_empty(),
            "Expected output from 'new' command"
        );

        // Step 2: Generate three additional marks using the 'next' subcommand
        // Step 2: Generate three additional marks using the 'next' subcommand
        // with sequential dates
        for i in 1..=3 {
            cargo_bin_cmd!("provenance")
                .arg("next")
                .arg("--date")
                .arg(format!("2023-06-{}T12:00:00Z", 20 + i))
                .arg("--comment")
                .arg(format!("Mark {}", i))
                .arg("--quiet")
                .assert()
                .success();
        }

        // Step 3: Validate all marks in the directory using 'validate --dir'
        let validate_output = cargo_bin_cmd!("provenance")
            .arg("validate")
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let validate_output_str = String::from_utf8_lossy(&validate_output);

        // Step 4: Expect the report will show no errors (empty output for
        // perfect chain)
        assert_actual_expected!(
            validate_output_str.trim(),
            "",
            "Expected empty output for a valid chain with no issues"
        );
    }

    #[test]
    fn test_validate_envelope_fixtures() {
        // Test the three fixtures from the prompt:
        // 1. ur:provenance - direct provenance mark
        // 2. ur:xid - XIDDocument with provenance assertion
        // 3. ur:envelope - envelope with provenance assertion

        let fixtures = [
            "ur:provenance/lfaohdftlrcydyoxwfwkolcnnswdzstyimctlyteehynhkckjynysthkdestnlutfmbshppmgmlsnesggltpspqzpfeemehlssgturbtkkfgtavawnwpfmkbkginlyisecvt",
            "ur:xid/tpsplstpsotanshdhdcxwsnyfhfdsgrtvyveptftfggdoeaaknldwmbyprvawebztkbyurinvlnltihfknbeoycsfzlftpsotngdgmgwhflfaxhdimbkfyndgyplolpkosdtbkcmdadyamincymdwnbsfrloglasmhwkrylkpklthttdzeecjtztjkvynnfsgadrhebdzswlinttsovtbdynrnotenzsflwzhlhfsrkewsehhkhhbnaseydtbkgavdienloemhgackbsesnsdpceghbachlyjpgafzdngronpabkheftfxhgeyrtdpnbgsmshglfoycsfylntpsohdcxbkfyndgyplolpkosdtbkcmdadyamincymdwnbsfrloglasmhwkrylkpklthttdzeoytpsoiajpihjktpsoaxoyadtpsojyjojpjlkoihjthsjtiaihdpioihjtihjphsjyjljpoytpsoisjtihksjydpjkihjstpsoadoytpsoinjpjtiodpjkjyhsjyihtpsohdcxiozeaaynkihyayjldaihcpwmolbdlapdlofhpfhlonuyaoktbbemcajtstjynelnoytpsoiejkihihietpsohdcxdlwzfnkkeylnuyrtbyqdsgytbtnlcskkylghclndehammekpaskbjsgyndahldjyoybstpsotansgmhdcxtojzpkgrtpoxseflttuyhpeemtttaakkjpcmieksdkiasnzsswiokgsgmujstedmoyaylstpsotansgylftanshfhdcxfwkeryktoncxzmaamnfgtpdybkwywlcywdrnvtceadlgtandmuahjnrezsuyaatotansgrhdcxssaakgiojebwdnolpdnswtsfzsrszsbtuepmlsdifeckckfdstlgbttersglwmbdoycsfncsfglfoycsfptpsotansgtlftansgohdcxcpvlsnwdrefscshyjemoltwydmvlmsskhtbgkbuecnpydsetttcamnfzmhoewepftansgehdcxdppsgaatpedsbzpllurtndhtmkmssnsfwkflytascsaeroaomkwzfwolglkghdweoybstpsotansgmhdcxftwecetnptptnydmoylokiwzteckleolbtaoftmsjlhdrtlffpdmtdmsjeglwtluwysfcnsr",
            "ur:envelope/lftpsojnghihjkjycxfejtkoihjzjljoihoycsfztpsotngdgmgwhflfaohdftlrcydyoxwfwkolcnnswdzstyimctlyteehynhkckjynysthkdestnlutfmbshppmgmlsnesggltpspqzpfeemehlssgturbtkkfgtavawnwpfmkbkginjzkehgyt",
        ];

        let ur_strings: Vec<String> =
            fixtures.iter().map(|s| s.to_string()).collect();

        // Validate with --warn flag since these are single marks without
        // genesis
        let (success, output) = run_validate_command(&ur_strings, true);

        assert!(
            success,
            "Validation should succeed for envelope fixtures. Output:\n{}",
            output
        );

        // Check that output contains expected information
        assert!(
            output.contains("Total marks: 2"),
            "Expected 2 distinct chains (ur:provenance and ur:envelope share same mark). Output:\n{}",
            output
        );
    }

    #[test]
    fn test_validate_direct_provenance_ur() {
        // Test just the direct ur:provenance fixture
        let ur_string = "ur:provenance/lfaohdftlrcydyoxwfwkolcnnswdzstyimctlyteehynhkckjynysthkdestnlutfmbshppmgmlsnesggltpspqzpfeemehlssgturbtkkfgtavawnwpfmkbkginlyisecvt";

        let (success, _output) =
            run_validate_command(&[ur_string.to_string()], true);

        assert!(
            success,
            "Validation should succeed for direct provenance UR"
        );
    }

    #[test]
    fn test_validate_xid_with_provenance() {
        // Test just the ur:xid fixture
        let ur_string = "ur:xid/tpsplstpsotanshdhdcxwsnyfhfdsgrtvyveptftfggdoeaaknldwmbyprvawebztkbyurinvlnltihfknbeoycsfzlftpsotngdgmgwhflfaxhdimbkfyndgyplolpkosdtbkcmdadyamincymdwnbsfrloglasmhwkrylkpklthttdzeecjtztjkvynnfsgadrhebdzswlinttsovtbdynrnotenzsflwzhlhfsrkewsehhkhhbnaseydtbkgavdienloemhgackbsesnsdpceghbachlyjpgafzdngronpabkheftfxhgeyrtdpnbgsmshglfoycsfylntpsohdcxbkfyndgyplolpkosdtbkcmdadyamincymdwnbsfrloglasmhwkrylkpklthttdzeoytpsoiajpihjktpsoaxoyadtpsojyjojpjlkoihjthsjtiaihdpioihjtihjphsjyjljpoytpsoisjtihksjydpjkihjstpsoadoytpsoinjpjtiodpjkjyhsjyihtpsohdcxiozeaaynkihyayjldaihcpwmolbdlapdlofhpfhlonuyaoktbbemcajtstjynelnoytpsoiejkihihietpsohdcxdlwzfnkkeylnuyrtbyqdsgytbtnlcskkylghclndehammekpaskbjsgyndahldjyoybstpsotansgmhdcxtojzpkgrtpoxseflttuyhpeemtttaakkjpcmieksdkiasnzsswiokgsgmujstedmoyaylstpsotansgylftanshfhdcxfwkeryktoncxzmaamnfgtpdybkwywlcywdrnvtceadlgtandmuahjnrezsuyaatotansgrhdcxssaakgiojebwdnolpdnswtsfzsrszsbtuepmlsdifeckckfdstlgbttersglwmbdoycsfncsfglfoycsfptpsotansgtlftansgohdcxcpvlsnwdrefscshyjemoltwydmvlmsskhtbgkbuecnpydsetttcamnfzmhoewepftansgehdcxdppsgaatpedsbzpllurtndhtmkmssnsfwkflytascsaeroaomkwzfwolglkghdweoybstpsotansgmhdcxftwecetnptptnydmoylokiwzteckleolbtaoftmsjlhdrtlffpdmtdmsjeglwtluwysfcnsr";

        let (success, _output) =
            run_validate_command(&[ur_string.to_string()], true);

        assert!(success, "Validation should succeed for XID with provenance");
    }

    #[test]
    fn test_validate_envelope_with_provenance() {
        // Test just the ur:envelope fixture
        let ur_string = "ur:envelope/lftpsojnghihjkjycxfejtkoihjzjljoihoycsfztpsotngdgmgwhflfaohdftlrcydyoxwfwkolcnnswdzstyimctlyteehynhkckjynysthkdestnlutfmbshppmgmlsnesggltpspqzpfeemehlssgturbtkkfgtavawnwpfmkbkginjzkehgyt";

        let (success, _output) =
            run_validate_command(&[ur_string.to_string()], true);

        assert!(
            success,
            "Validation should succeed for envelope with provenance"
        );
    }

    #[test]
    fn test_validate_envelope_without_provenance_fails() {
        // Create an envelope without a provenance assertion - should fail
        let ur_string = "ur:envelope/tpsotpsojnghihjkjycxfejtkohsjljpcxjyhsjljptpsoioihcxfejtihjyisihjkjpiehsjyjlcxjyhsjljpaatpsojojyhsjyjljtfloxlrashhbdcx";

        let (success, _output) =
            run_validate_command(&[ur_string.to_string()], false);

        assert!(
            !success,
            "Validation should fail for envelope without provenance assertion"
        );
    }
}
