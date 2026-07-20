import unittest

from cli.cli import build_parser


class VerifyCliTests(unittest.TestCase):
    def test_verify_uses_test_build_by_default(self) -> None:
        args = build_parser().parse_args(["verify", "example-crate"])

        self.assertTrue(args.test)

    def test_verify_can_disable_test_build(self) -> None:
        args = build_parser().parse_args(["verify", "example-crate", "--no-test"])

        self.assertFalse(args.test)


if __name__ == "__main__":
    unittest.main()
