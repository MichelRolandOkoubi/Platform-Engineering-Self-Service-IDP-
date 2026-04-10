terraform {
  backend "s3" {
    bucket = "idp-platform-terraform-state"
    key    = "global/s3/terraform.tfstate"
    region = "us-east-1"
  }
}
