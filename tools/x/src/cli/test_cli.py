import unittest

from cli.cli import build_parser


class VerifyCliTests(unittest.TestCase):
    def test_verify_uses_test_build_by_default(self) -> None:
        args = build_parser().parse_args(["verify", "example-crate"])

        self.assertTrue(args.test)

    def test_verify_can_disable_test_build(self) -> None:
        args = build_parser().parse_args(["verify", "example-crate", "--no-test"])

        self.assertFalse(args.test)


class ResultCliTests(unittest.TestCase):
    def test_result_accepts_dir(self) -> None:
        args = build_parser().parse_args(["result", "-dir", "one-run"])

        self.assertEqual(args.result_dir, "one-run")
        self.assertIsNone(args.result_dirdir)
        self.assertEqual(args.granularity, "rule")

    def test_result_accepts_dirdir(self) -> None:
        args = build_parser().parse_args(["result", "-dirdir", "many-runs"])

        self.assertIsNone(args.result_dir)
        self.assertEqual(args.result_dirdir, "many-runs")

    def test_result_accepts_caller_granularity(self) -> None:
        args = build_parser().parse_args(
            ["result", "--granularity", "caller"]
        )

        self.assertEqual(args.granularity, "caller")

    def test_result_accepts_unsafe_api_summary(self) -> None:
        args = build_parser().parse_args(["result", "--unsafe-api"])

        self.assertTrue(args.unsafe_api_summary)


if __name__ == "__main__":
    unittest.main()
