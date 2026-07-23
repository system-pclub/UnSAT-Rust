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

    def test_result_accepts_dirdir(self) -> None:
        args = build_parser().parse_args(["result", "-dirdir", "many-runs"])

        self.assertIsNone(args.result_dir)
        self.assertEqual(args.result_dirdir, "many-runs")


if __name__ == "__main__":
    unittest.main()
