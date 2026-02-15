import { invoke } from "@tauri-apps/api/core";
import type {
  VaultDto,
  QuestionnaireImportDto,
  ColumnMapDto,
  AnswerBankEntryDto,
  AnswerBankCreateInputDto,
  AnswerBankUpdatePatchDto,
  MatchSuggestionDto,
  LicenseStatusDto,
} from "@packages/types";

// ============================================================================
// VAULT COMMANDS
// ============================================================================

export async function invokeVaultCreate(path: string, name: string): Promise<VaultDto> {
  return invoke("vault_create", { path, name });
}

export async function invokeVaultOpen(path: string): Promise<VaultDto> {
  return invoke("vault_open", { path });
}

export async function invokeVaultClose(): Promise<void> {
  return invoke("vault_close");
}

export async function invokeVaultLock(): Promise<void> {
  return invoke("vault_lock");
}

// ============================================================================
// QUESTIONNAIRE COMMANDS
// ============================================================================

export interface ColumnProfileDto {
  col_ref: string;
  ordinal: number;
  label: string;
  non_empty_count: number;
  sample: string[];
}

export async function invokeImportQuestionnaire(filePath: string): Promise<QuestionnaireImportDto> {
  return invoke("import_questionnaire", { file_path: filePath });
}

export async function invokeGetColumnProfiles(importId: string): Promise<ColumnProfileDto[]> {
  return invoke("get_column_profiles", { import_id: importId });
}

export async function invokeSaveColumnMapping(
  importId: string,
  columnMap: ColumnMapDto
): Promise<QuestionnaireImportDto> {
  return invoke("save_column_mapping", {
    import_id: importId,
    column_map: columnMap,
  });
}

// ============================================================================
// ANSWER BANK COMMANDS
// ============================================================================

export async function invokeAnswerBankCreate(
  input: AnswerBankCreateInputDto
): Promise<AnswerBankEntryDto> {
  return invoke("answer_bank_create", { input });
}

export async function invokeAnswerBankUpdate(
  entryId: string,
  patch: AnswerBankUpdatePatchDto
): Promise<AnswerBankEntryDto> {
  return invoke("answer_bank_update", { entry_id: entryId, patch });
}

export async function invokeAnswerBankDelete(entryId: string): Promise<void> {
  return invoke("answer_bank_delete", { entry_id: entryId });
}

export async function invokeAnswerBankList(
  limit: number,
  offset: number
): Promise<AnswerBankEntryDto[]> {
  return invoke("answer_bank_list", { limit, offset });
}

// ============================================================================
// MATCHING COMMANDS
// ============================================================================

export async function invokeGetMatchingSuggestions(
  question: string,
  topN?: number
): Promise<MatchSuggestionDto[]> {
  return invoke("get_matching_suggestions", {
    question,
    top_n: topN ?? 5,
  });
}

// ============================================================================
// EXPORT COMMANDS
// ============================================================================

export interface ExportPackDto {
  zip_path: string;
  manifest_version: number;
  file_count: number;
}

export async function invokeGenerateExportPack(outputPath: string): Promise<ExportPackDto> {
  return invoke("generate_export_pack", { output_path: outputPath });
}

// ============================================================================
// LICENSE COMMANDS
// ============================================================================

export async function invokeCheckLicenseStatus(): Promise<LicenseStatusDto> {
  return invoke("check_license_status");
}

export async function invokeInstallLicense(licensePath: string): Promise<LicenseStatusDto> {
  return invoke("install_license", { license_path: licensePath });
}
