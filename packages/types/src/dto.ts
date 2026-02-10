// DTO definitions mirrored from Rust. This is intentionally minimal in Phase 0/1.

export type VaultDto = {
  vault_id: string;
  name: string;
  root_path: string;
  created_at: string;
  encryption_mode: 'none' | 'passphrase';
  schema_version: number;
};

export type EvidenceDto = {
  evidence_id: string;
  vault_id: string;
  filename: string;
  relative_path: string;
  content_type: string;
  byte_size: number;
  sha256: string;
  source: 'manual_import' | 'generated' | 'extracted';
  tags: string[];
  created_at: string;
  notes?: string;
};
